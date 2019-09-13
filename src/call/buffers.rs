// Copyright 2019 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::{self, BufRead, ErrorKind, Read, Write};
use std::{cmp, mem, ptr, usize};

use crate::grpc_sys::{
    self, grpc_byte_buffer_reader, grpc_slice,
};
#[cfg(feature = "prost-codec")]
use bytes::{Buf, BufMut};

pub struct GrpcByteBuffer {
    pub raw: *mut grpc_sys::grpc_byte_buffer,
}

impl GrpcByteBuffer {
    pub fn push(&mut self, slice: grpc_slice) {
        unsafe {
            grpc_sys::grpcwrap_byte_buffer_add(self.raw as _, slice);
        }
    }

    pub fn pop(&mut self) {
        unsafe { grpc_sys::grpcwrap_byte_buffer_pop(self.raw as _) }
    }

    pub fn clear(&mut self) {
        unsafe { grpc_sys::grpcwrap_byte_buffer_reset_and_unref(self.raw as _) }
    }

    pub fn len(&self) -> usize {
        unsafe { grpc_sys::grpc_byte_buffer_length(self.raw) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Increase the ref count, so it's mutated.
    pub fn clone(&mut self) -> Self {
        unsafe {
            GrpcByteBuffer {
                raw: grpc_sys::grpc_byte_buffer_copy(self.raw),
            }
        }
    }

    pub unsafe fn take_raw(&mut self) -> *mut grpc_sys::grpc_byte_buffer {
        let ret = self.raw;
        self.raw = grpc_sys::grpc_raw_byte_buffer_create(ptr::null_mut(), 0);
        ret
    }
}

unsafe impl Send for GrpcByteBuffer {}

impl Default for GrpcByteBuffer {
    fn default() -> Self {
        unsafe {
            GrpcByteBuffer {
                raw: grpc_sys::grpc_raw_byte_buffer_create(ptr::null_mut(), 0),
            }
        }
    }
}

impl<'a> From<&'a mut GrpcByteBuffer> for grpc_byte_buffer_reader {
    fn from(src: &'a mut GrpcByteBuffer) -> Self {
        let mut reader;
        unsafe {
            reader = mem::zeroed();
            let init_result = grpc_sys::grpc_byte_buffer_reader_init(&mut reader, src.raw);
            assert_eq!(init_result, 1);
        }
        reader
    }
}

impl<'a> From<&'a mut [grpc_slice]> for GrpcByteBuffer {
    fn from(slice: &'a mut [grpc_slice]) -> Self {
        unsafe {
            GrpcByteBuffer {
                raw: grpc_sys::grpc_raw_byte_buffer_create(slice.as_mut_ptr(), slice.len()),
            }
        }
    }
}

impl Drop for GrpcByteBuffer {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_byte_buffer_destroy(self.raw) }
    }
}

/// `MessageReader` is a zero-copy reader for the message payload.
///
/// To achieve zero-copy, use the BufRead API `fill_buf` and `consume`
/// to operate the reader.
pub struct MessageReader {
    _buf: GrpcByteBuffer,
    reader: grpc_byte_buffer_reader,
    buffer_slice: grpc_slice,
    buffer_offset: usize,
    length: usize,
}

impl MessageReader {
    /// Get the available bytes count of the reader.
    #[inline]
    pub fn pending_bytes_count(&self) -> usize {
        self.length
    }

    pub fn new(buf: GrpcByteBuffer, reader: grpc_byte_buffer_reader, length: usize) -> MessageReader {
        MessageReader {
            _buf: buf,
            reader,
            buffer_slice: Default::default(),
            buffer_offset: 0,
            length,
        }
    }
}

unsafe impl Sync for MessageReader {}
unsafe impl Send for MessageReader {}

impl Read for MessageReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let amt = {
            let bytes = self.fill_buf()?;
            if bytes.is_empty() {
                return Ok(0);
            }
            let amt = cmp::min(buf.len(), bytes.len());
            buf[..amt].copy_from_slice(&bytes[..amt]);
            amt
        };
        self.consume(amt);
        Ok(amt)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        if self.length == 0 {
            return Ok(0);
        }
        buf.reserve(self.length);
        let start = buf.len();
        let mut len = start;
        unsafe {
            buf.set_len(start + self.length);
        }
        let ret = loop {
            match self.read(&mut buf[len..]) {
                Ok(0) => break Ok(len - start),
                Ok(n) => len += n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => break Err(e),
            }
        };
        unsafe {
            buf.set_len(len);
        }
        ret
    }
}

impl BufRead for MessageReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        // Optimization for empty slice
        if self.pending_bytes_count() == 0 {
            return Ok(&[]);
        }

        // When finished reading current `buffer_slice`, start reading next slice
        let buffer_len = self.buffer_slice.len();
        if buffer_len == 0 || self.buffer_offset == buffer_len {
            self.buffer_slice = self.reader.next_slice();
            self.buffer_offset = 0;
        }

        debug_assert!(self.buffer_offset <= buffer_len);
        Ok(self.buffer_slice.range_from(self.buffer_offset))
    }

    fn consume(&mut self, amt: usize) {
        self.length -= amt;
        self.buffer_offset += amt;
    }
}

impl Drop for MessageReader {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_byte_buffer_reader_destroy(&mut self.reader);
        }
    }
}

#[cfg(feature = "prost-codec")]
impl Buf for MessageReader {
    fn remaining(&self) -> usize {
        self.pending_bytes_count()
    }

    fn bytes(&self) -> &[u8] {
        // This is similar but not identical to `BuffRead::fill_buf`, since `self`
        // is not mutable, we can only return bytes up to the end of the current
        // slice.

        // Optimization for empty slice
        if self.buffer_slice.is_empty() {
            return &[];
        }

        debug_assert!(self.buffer_offset <= self.buffer_slice.len());
        self.buffer_slice.range_from(self.buffer_offset)
    }

    fn advance(&mut self, mut cnt: usize) {
        // Similar but not identical to `BufRead::consume`. We must also advance
        // the buffer slice if we have exhausted the current slice.

        // The number of bytes remaining in the current slice.
        let mut remaining = self.buffer_slice.len() - self.buffer_offset;
        while remaining <= cnt {
            self.consume(remaining);
            if self.pending_bytes_count() == 0 {
                return;
            }

            cnt -= remaining;
            self.buffer_slice = self.reader.next_slice();
            self.buffer_offset = 0;
            remaining = self.buffer_slice.len();
        }

        self.consume(cnt);
    }
}

pub struct GrpcSliceBuffer {
    buffer: grpc_slice,
    buffer_offset: usize,
}

impl GrpcSliceBuffer {
    pub fn is_full(&self) -> bool {
        self.buffer.len() - self.buffer_offset == 0
    }

    /// Returns the remaining slice, `None` means fully consumed
    pub fn append<'a>(&mut self, data: &'a [u8]) -> Option<&'a [u8]> {
        let internal_slice = unsafe { self.buffer.range_from_unsafe(self.buffer_offset) };
        let data_len = data.len();
        let internal_len = internal_slice.len();
        if data_len > internal_len {
            self.buffer_offset += internal_len;
            internal_slice.copy_from_slice(&data[..internal_len]);
            Some(&data[internal_len..])
        } else {
            self.buffer_offset += data_len;
            internal_slice[..data_len].copy_from_slice(data);
            None
        }
    }
}

unsafe impl Send for GrpcSliceBuffer {}

#[cfg(feature = "prost-codec")]
impl BufMut for GrpcSliceBuffer {
    fn remaining_mut(&self) -> usize {
        self.buffer.len() - self.buffer_offset
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buffer_offset += cnt
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        self.buffer.range_from_unsafe(self.buffer_offset)
    }
}

pub struct MessageWriter {
    data: GrpcByteBuffer,
    reserved_buffer: Option<GrpcSliceBuffer>,
    size: usize,
}

impl MessageWriter {
    pub fn new() -> MessageWriter {
        MessageWriter {
            data: Default::default(),
            reserved_buffer: None,
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }
        self.data.clear();
        self.size = 0;
    }

    pub fn reserve(&mut self, size: usize) {
        if size <= self.size {
            return;
        }
        self.flush().unwrap();
        // `self.reserved_buffer` is supposed to be None after `self.flush()`
        debug_assert!(self.reserved_buffer.is_none());
        let new_size = size - self.size;
        let buffer = grpc_slice::with_capacity(new_size);
        self.reserved_buffer = Some(GrpcSliceBuffer {
            buffer,
            buffer_offset: 0,
        })
    }

    pub fn into_buffer(self) -> GrpcByteBuffer {
        self.data
    }

    pub fn as_buffer(&mut self) -> &mut GrpcByteBuffer {
        &mut self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.reserved_buffer
            .as_ref()
            .map_or(0, |buf| buf.buffer_offset)
            + self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn append_slice_to_data(&mut self, slice: grpc_slice) {
        self.size += slice.len();
        self.data.push(slice);
    }

    fn append_buf_to_reserved<'a>(&mut self, buf: &'a [u8]) -> Option<&'a [u8]> {
        use std::mem::swap;
        let mut dummy_buffer = None;
        swap(&mut dummy_buffer, &mut self.reserved_buffer);
        match dummy_buffer {
            Some(mut buffer) => {
                let rest = buffer.append(buf);
                if buffer.is_full() {
                    // Full, push it into the buffer
                    self.append_slice_to_data(buffer.buffer);
                } else {
                    // Not full, put it back to `self`
                    self.reserved_buffer = Some(buffer);
                }
                rest
            }
            None => Some(buf),
        }
    }

    /// Returns the rest
    pub fn write_safe(&mut self, buf: &[u8]) {
        if let Some(rest) = self.append_buf_to_reserved(buf) {
            self.append_slice_to_data(From::from(rest));
        }
    }
}

impl Write for MessageWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_safe(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        use std::mem::swap;
        let mut dummy_buffer = None;
        swap(&mut dummy_buffer, &mut self.reserved_buffer);

        if let Some(buffer) = dummy_buffer {
            // 0-sized buffers shouldn't haven been created
            debug_assert!(buffer.buffer_offset > 0);
            self.append_slice_to_data(if buffer.is_full() {
                // Current buffer is filled
                buffer.buffer
            } else {
                From::from(buffer.buffer.range_to(buffer.buffer_offset))
            });
        }
        Ok(())
    }
}

#[cfg(feature = "prost-codec")]
impl BufMut for MessageWriter {
    fn remaining_mut(&self) -> usize {
        self.reserved_buffer
            .as_ref()
            .map_or(0, |buf| buf.remaining_mut())
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        if let Some(buf) = &mut self.reserved_buffer {
            buf.advance_mut(cnt)
        }
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        self.reserved_buffer
            .as_mut()
            .map_or(&mut [], |buf| buf.bytes_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_buffer_empty() {
        let mut buf = GrpcByteBuffer::default();
        unsafe {
            assert_eq!(
                0,
                GrpcByteBuffer {
                    raw: buf.take_raw(),
                }
                .len()
            );
        }
        assert_eq!(0, buf.len());
    }

    #[test]
    fn byte_buffer_clear_after_taken_away() {
        let mut buf = GrpcByteBuffer::default();
        let data = "oh my god!".as_bytes();
        buf.push(From::from(data));
        unsafe {
            assert_eq!(
                data.len(),
                GrpcByteBuffer {
                    raw: buf.take_raw(),
                }
                .len()
            );
        }
        buf.clear();
        assert_eq!(0, buf.len());
    }

    #[test]
    fn byte_buffer_clear_empty() {
        let mut buf = GrpcByteBuffer::default();
        buf.clear();
        buf.clear();
        buf.push(From::from("bla".as_bytes()));
        buf.push(From::from("bla".as_bytes()));
        buf.clear();
        buf.clear();
        buf.push(From::from("bla".as_bytes()));
        buf.push(From::from("bla".as_bytes()));
        buf.clear();
        buf.clear();
    }

    #[test]
    fn byte_buffer_simple() {
        let mut buf = GrpcByteBuffer::default();
        assert_eq!(0, buf.len());
        let data = "2333".as_bytes();
        buf.push(From::from(data));
        assert_eq!(data.len(), buf.len());
        let data1 = "666".as_bytes();
        buf.push(From::from(data1));
        assert_eq!(data.len() + data1.len(), buf.len());
        buf.clear();
        assert_eq!(0, buf.len());
        buf.push(From::from(data));
        assert_eq!(data.len(), buf.len());
        buf.push(From::from(data1));
        assert_eq!(data.len() + data1.len(), buf.len());
    }

    fn make_message_reader(source: &[u8], n_slice: usize) -> MessageReader {
        let mut slices = vec![From::from(source); n_slice];
        let mut buf = GrpcByteBuffer::from(slices.as_mut_slice());
        let reader = grpc_byte_buffer_reader::from(&mut buf);
        let length = reader.len();

        MessageReader {
            _buf: buf,
            reader,
            buffer_slice: Default::default(),
            buffer_offset: 0,
            length,
        }
    }

    #[test]
    // Old code crashes under a very weird circumstance, due to a typo in `MessageReader::consume`
    fn test_typo_len_offset() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // half of the size of `data`
        const HALF_SIZE: usize = 4;
        let mut reader = make_message_reader(&data, 1);
        assert_eq!(reader.pending_bytes_count(), data.len());
        // first 3 elements of `data`
        let mut buf = [0; HALF_SIZE];
        reader.read(&mut buf).unwrap();
        assert_eq!(data[..HALF_SIZE], buf);
        reader.read(&mut buf).unwrap();
        assert_eq!(data[HALF_SIZE..], buf);
    }

    #[test]
    fn test_message_reader() {
        for len in 0..1024 + 1 {
            for n_slice in 1..4 {
                let source = vec![len as u8; len];
                let expect = vec![len as u8; len * n_slice];
                // Test read.
                let mut reader = make_message_reader(&source, n_slice);
                let mut dest = [0; 7];
                let amt = reader.read(&mut dest).unwrap();

                assert_eq!(
                    dest[..amt],
                    expect[..amt],
                    "len: {}, nslice: {}",
                    len,
                    n_slice
                );

                // Read after move.
                let mut box_reader = Box::new(reader);
                let amt = box_reader.read(&mut dest).unwrap();
                assert_eq!(
                    dest[..amt],
                    expect[..amt],
                    "len: {}, nslice: {}",
                    len,
                    n_slice
                );

                // Test read_to_end.
                let mut reader = make_message_reader(&source, n_slice);
                let mut dest = vec![];
                reader.read_to_end(&mut dest).unwrap();
                assert_eq!(dest, expect, "len: {}, nslice: {}", len, n_slice);

                assert_eq!(0, reader.pending_bytes_count());
                assert_eq!(0, reader.read(&mut [1]).unwrap())
            }
        }
    }

    #[cfg(feature = "prost-codec")]
    #[test]
    fn test_buf_impl() {
        for len in 0..1024 + 1 {
            for n_slice in 1..4 {
                let source = vec![len as u8; len];

                let mut reader = make_message_reader(&source, n_slice);

                let mut remaining = len * n_slice;
                let mut count = 100;
                while reader.remaining() > 0 {
                    assert_eq!(remaining, reader.remaining());
                    let bytes = Buf::bytes(&reader);
                    bytes.iter().for_each(|b| assert_eq!(*b, len as u8));
                    let mut read = bytes.len();
                    // We don't have to advance by the whole amount we read.
                    if read > 5 && len % 2 == 0 {
                        read -= 5;
                    }
                    reader.advance(read);
                    remaining -= read;
                    count -= 1;
                    assert!(count > 0);
                }

                assert_eq!(0, remaining);
                assert_eq!(0, reader.remaining());
            }
        }
    }

    #[test]
    fn test_slice_buffer() {
        let mut buffer = GrpcSliceBuffer {
            buffer: grpc_slice::with_capacity(5),
            buffer_offset: 0,
        };
        let should_be_none = buffer.append("Ping".as_bytes());
        assert_eq!(should_be_none, None);
        let should_be_ap = buffer.append("CAP".as_bytes());
        assert_eq!(should_be_ap, Some("AP".as_bytes()));
    }

    #[test]
    fn test_message_writer() {
        let mut writer = MessageWriter::new();
        assert_eq!(writer.len(), 0);
        writer.write_safe("114".as_bytes());
        assert_eq!(writer.len(), 3);
        writer.write("514".as_bytes()).unwrap();
        assert_eq!(writer.len(), 6);
        assert_eq!(writer.as_buffer().len(), 6);
    }

    #[test]
    fn test_message_writer_reserve() {
        let mut writer = MessageWriter::new();
        writer.reserve(3);
        writer.write_safe(&[1]);
        // Longer than 2
        let text = "TiDB will rule the world!".as_bytes();
        writer.write(text).unwrap();
        assert_eq!(writer.as_buffer().len(), text.len() + 1);
    }
}
