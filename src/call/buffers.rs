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

use std::io::{self, BufRead, ErrorKind, Read};
use std::mem::MaybeUninit;
use std::{cmp, mem, usize};

use crate::grpc_sys::{self, grpc_byte_buffer, grpc_slice_buffer, GrpcByteBufferReader, GrpcSlice};

#[cfg(feature = "prost-codec")]
use bytes::{Buf, BufMut};

/// `MessageReader` is a zero-copy reader for the message payload.
///
/// To achieve zero-copy, use the BufRead API `fill_buf` and `consume`
/// to operate the reader.
pub struct MessageReader {
    reader: GrpcByteBufferReader,
    buffer_slice: GrpcSlice,
    buffer_offset: usize,
    length: usize,
}

impl MessageReader {
    /// Get the available bytes count of the reader.
    #[inline]
    fn pending_bytes_count(&self) -> usize {
        self.length
    }

    /// Create a new `MessageReader`.
    ///
    /// Safety: `raw` must be a unique reference.
    pub unsafe fn new(raw: *mut grpc_byte_buffer) -> MessageReader {
        let reader = GrpcByteBufferReader::new(raw);
        let length = reader.len();

        MessageReader {
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

/// Wraps a `grpc_slice_buffer` and provides an intermediate write buffer.
pub struct MessageWriter {
    // A `grpc_slice_buffer` cannot be moved, so we must keep it on the heap and
    // never move it until we are dropped.
    // FIXME: it *might* be quicker to arena allocate these.
    buffer: Box<grpc_slice_buffer>,
    write_buffer: Option<Vec<u8>>,
}

impl MessageWriter {
    /// Create an empty MessageWriter.
    pub fn new() -> MessageWriter {
        let buffer = unsafe {
            // This unsafe block is just to let grpc initialise `buffer`.
            let mut buffer = Box::new(MaybeUninit::uninit());
            grpc_sys::grpc_slice_buffer_init(buffer.as_mut_ptr());
            mem::transmute::<_, Box<grpc_slice_buffer>>(buffer)
        };

        MessageWriter {
            buffer,
            write_buffer: None,
        }
    }

    /// Clear the message writer. Any data that has not been flushed will be lost.
    /// (And will cause a panic in debug builds).
    pub fn clear(&mut self) {
        debug_assert!(self.write_buffer.is_none());
        unsafe {
            grpc_sys::grpc_slice_buffer_reset_and_unref(&mut *self.buffer);
        }
    }

    /// Flush data written. Creates a new slice in the internal `grpc_slice_buffer`.
    pub fn flush(&mut self) {
        // Any data written to `write_buffer` is saved into `buffer`.
        if let Some(mut buf) = self.write_buffer.take() {
            unsafe {
                let cap = buf.capacity();
                let ptr = buf.as_mut_ptr();
                mem::forget(buf);
                let slice =
                    grpc_sys::grpc_slice_new_with_len(ptr as *mut _, cap, Some(destroy_slice));
                grpc_sys::grpc_slice_buffer_add(&mut *self.buffer, slice);
            }
        }
    }

    /// If you `reserve`, you must flush or you will lose any data written. Write
    /// to the returned `Vec`.
    pub fn reserve(&mut self, size: usize) -> &mut [u8] {
        debug_assert!(
            self.write_buffer.is_none(),
            "Any data in write_buffer will be lost"
        );
        self.write_buffer = Some(Vec::with_capacity(size));
        let result = self.write_buffer.as_mut().unwrap();
        result.resize(result.capacity(), 0);
        result
    }

    // Unsafe because the caller takes responsibility for destroying the returned
    // byte buffer.
    pub unsafe fn byte_buffer(&self) -> *mut grpc_byte_buffer {
        debug_assert!(self.write_buffer.is_none());
        grpc_sys::grpc_raw_byte_buffer_create(self.buffer.slices, self.buffer.count)
    }
}

impl Drop for MessageWriter {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_slice_buffer_destroy(&mut *self.buffer);
        }
    }
}

// A function used to tidy up a `grpc_slice` which wraps a Rust `Vec`.
extern "C" fn destroy_slice(ptr: *mut ::std::os::raw::c_void, cap: usize) {
    unsafe {
        let vec = Vec::from_raw_parts(ptr as *mut u8, cap, cap);
        mem::drop(vec);
    }
}

// Safe because both `grpc_slice_buffer` and `Vec` are safe to send across threads
// as long as only one thread has access.
unsafe impl Send for MessageWriter {}

/// A wrapper for `MessageWriter` for implementing `Bytes::BufMut`. A wrapper is
/// needed because `BufMut` can be read and written incrementally, which
/// `MessageWriter` does not support.
#[cfg(feature = "prost-codec")]
pub struct MessageWriterBuf<'a> {
    inner: &'a mut MessageWriter,
    offset: usize,
}

#[cfg(feature = "prost-codec")]
impl<'a> MessageWriterBuf<'a> {
    /// Flush the inner `MessageWriter`.
    pub fn flush(&mut self) {
        self.inner.flush()
    }
}

#[cfg(feature = "prost-codec")]
impl<'a> From<&'a mut MessageWriter> for MessageWriterBuf<'a> {
    fn from(inner: &'a mut MessageWriter) -> MessageWriterBuf<'a> {
        MessageWriterBuf { inner, offset: 0 }
    }
}

#[cfg(feature = "prost-codec")]
impl<'a> BufMut for MessageWriterBuf<'a> {
    fn remaining_mut(&self) -> usize {
        match &self.inner.write_buffer {
            Some(v) => v.len() - self.offset,
            None => 0,
        }
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.offset += cnt;
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        // If the user has not previously called `reserve`, then we will
        // return an empty array.
        match &mut self.inner.write_buffer {
            Some(v) => &mut v[self.offset..],
            None => &mut [],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MessageWriter {
        // Only returns the length of flushed bytes.
        fn len(&self) -> usize {
            unsafe {
                let bb = self.byte_buffer();
                let len = grpc_sys::grpc_byte_buffer_length(bb);
                grpc_sys::grpc_byte_buffer_destroy(bb);
                len
            }
        }
    }

    fn make_message_reader(source: &[u8], n_slice: usize) -> MessageReader {
        unsafe {
            let mut data: Vec<_> = ::std::iter::repeat(source)
                .take(n_slice)
                .map(|s| grpc_sys::grpc_slice_from_copied_buffer(s.as_ptr() as _, s.len()))
                .collect();
            let buf = grpc_sys::grpc_raw_byte_buffer_create(data.as_mut_ptr(), data.len());
            MessageReader::new(buf)
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
    fn msg_writer_reserve_flush_clear() {
        let mut writer = MessageWriter::new();
        assert_eq!(writer.len(), 0);
        let bytes = writer.reserve(3);
        bytes[2] = 42;
        writer.flush();
        assert_eq!(writer.len(), 3);
        writer.clear();
        assert_eq!(writer.len(), 0);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn msg_writer_no_flush() {
        let mut writer = MessageWriter::new();
        assert_eq!(writer.len(), 0);
        let bytes = writer.reserve(10);
        bytes[0] = 42;
        writer.clear();
    }

    #[test]
    fn msg_writer_multi_write() {
        let mut writer = MessageWriter::new();
        assert_eq!(writer.len(), 0);
        let bytes = writer.reserve(3);
        bytes[0] = 42;
        writer.flush();
        let bytes = writer.reserve(3);
        bytes[2] = 255;
        writer.flush();
        let bytes = writer.reserve(2);
        bytes[1] = 0;
        writer.flush();
        assert_eq!(writer.len(), 8);
    }

    #[cfg(feature = "prost-codec")]
    #[test]
    fn msg_writer_buf_mut() {
        let writer = &mut MessageWriter::new();
        assert_eq!(writer.len(), 0);
        writer.reserve(10);
        unsafe {
            let mut buf: MessageWriterBuf = writer.into();
            assert_eq!(buf.remaining_mut(), 10);
            let bytes = buf.bytes_mut();
            bytes[0] = 4;
            bytes[3] = 42;
            buf.advance_mut(3);
            assert_eq!(buf.remaining_mut(), 7);
            assert_eq!(buf.bytes_mut()[0], 42);
        }
    }
}
