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

//! This module provides data structures for reading and writing client data.

use std::{
    cmp,
    io::{self, BufRead, ErrorKind, Read},
    marker::PhantomPinned,
    mem::{self, MaybeUninit},
    ptr, slice, usize,
};

use crate::grpc_sys::{
    self, grpc_byte_buffer, grpc_byte_buffer_reader, grpc_slice, grpc_slice_refcount,
    grpc_slice_refcount_vtable,
};

#[cfg(feature = "prost-codec")]
use bytes::{Buf, BufMut};

// A wrapper for `grpc_slice`.
struct GrpcSlice(grpc_slice);

impl GrpcSlice {
    fn len(&self) -> usize {
        unsafe { grpc_sys::grpcwrap_slice_length(&self.0) }
    }

    fn range_from(&self, offset: usize) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = grpc_sys::grpcwrap_slice_raw_offset(&self.0, offset, &mut len);
            slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl Default for GrpcSlice {
    fn default() -> Self {
        GrpcSlice(unsafe { grpc_sys::grpc_empty_slice() })
    }
}

impl Drop for GrpcSlice {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpcwrap_slice_unref(&self.0);
        }
    }
}

// A wrapper for `grpc_byte_buffer_reader`.
//
// Manages the `byte_buffer` behind the reader as well as the reader itself. That
// means we expect a 1:1 relationship between the `byte_buffer` and reader, so
// this is not a general purpose wrapper.
struct GrpcByteBufferReader(grpc_byte_buffer_reader);

impl GrpcByteBufferReader {
    // Takes ownership of `buf` and will destroy it (at some point). The caller
    // must not keep a reference.
    //
    // Safety: `buf` must be valid and non-null. The caller must not keep a reference
    // to it.
    unsafe fn new(buf: *mut grpc_byte_buffer) -> GrpcByteBufferReader {
        let mut reader = MaybeUninit::uninit();
        let init_result = grpc_sys::grpc_byte_buffer_reader_init(reader.as_mut_ptr(), buf);
        assert_eq!(init_result, 1);
        GrpcByteBufferReader(reader.assume_init())
    }

    fn len(&self) -> usize {
        unsafe { grpc_sys::grpc_byte_buffer_length(self.0.buffer_out) }
    }

    fn next_slice(&mut self) -> GrpcSlice {
        unsafe {
            let mut slice = GrpcSlice::default();
            let code = grpc_sys::grpc_byte_buffer_reader_next(&mut self.0, &mut slice.0);
            debug_assert_ne!(code, 0);
            slice
        }
    }
}

impl Drop for GrpcByteBufferReader {
    fn drop(&mut self) {
        unsafe {
            let buf = self.0.buffer_in;
            grpc_sys::grpc_byte_buffer_reader_destroy(&mut self.0);
            grpc_sys::grpc_byte_buffer_destroy(buf);
        }
    }
}

// Constructor function for a `grpc_slice` which wraps a Rust `Vec`.
//
// Safety: see `VecSliceRefCount::new`, the same applies here.
unsafe fn vec_slice(v: Vec<u8>) -> Box<grpc_slice> {
    let mut data = grpc_sys::grpc_slice_grpc_slice_data::default();
    *data.refcounted.as_mut() = grpc_sys::grpc_slice_grpc_slice_data_grpc_slice_refcounted {
        bytes: v.as_ptr() as *const _ as *mut _,
        length: v.len(),
    };
    let mut refcount = VecSliceRefCount::new(v);
    let mut result = Box::new(grpc_slice {
        refcount: ptr::null_mut(),
        data,
    });
    refcount.slice = result.as_mut();
    result.refcount = Box::into_raw(refcount) as *mut _;
    result
}

// comment: grpc_slice_refcount, sub_refcount
// A `grpc_slice_refcount` structure to handle ref-counting and memory mangement
// of `grpc_slice`s created by `vec_slice`.
//
// Note that `grpc_slice_refcount` must be a prefix of this type so that a pointer
// to `VecSliceRefCount` can be treated polymorphically as a pointer to
// `grpc_slice_refcount`.
#[repr(C)]
struct VecSliceRefCount {
    // 'vtable' pointer, should always point at VEC_SLICE_VTABLE.
    vtable: *const grpc_slice_refcount_vtable,
    // Self-reference. Must not be null.
    sub_refcount: *mut grpc_slice_refcount,
    // The `Vec` providing the memory for this slice.
    vec: Vec<u8>,
    // Refcount.
    count: usize,
    // Pointer to the slice this object is managing.
    slice: *mut grpc_slice,
    // Because `sub_refcount` is self-referential.
    _phantom: PhantomPinned,
}

impl VecSliceRefCount {
    // Create a new `VecSliceRefCount`.
    //
    // Safety: the returned `VecSliceRefCount` has a 0 refcount. It is the
    // caller's responsibility to increment the refcount or either memory will
    // be leaked or `vec_slice_unref` will panic.
    //
    // `VecSliceRefCount::sub_refcount` is self-referential, therefore a
    // `VecSliceRefCount` *must never be moved*.
    unsafe fn new(vec: Vec<u8>) -> Box<VecSliceRefCount> {
        let mut result = Box::new(VecSliceRefCount {
            vtable: &VEC_SLICE_VTABLE,
            sub_refcount: ptr::null_mut(),
            vec,
            count: 0,
            slice: ptr::null_mut(),
            _phantom: PhantomPinned,
        });
        result.sub_refcount = &*result as *const _ as *mut _;
        result
    }
}

// gRPC data structure proving functions for managing a `grpc_slice` created by
// `vec_slice`.
static VEC_SLICE_VTABLE: grpc_slice_refcount_vtable = grpc_slice_refcount_vtable {
    ref_: Some(vec_slice_ref),
    unref: Some(vec_slice_unref),
    eq: Some(grpc_sys::grpc_slice_default_eq_impl),
    hash: Some(grpc_sys::grpc_slice_default_hash_impl),
};

// Increment a vec_slice ref count.
unsafe extern "C" fn vec_slice_ref(arg1: *mut ::std::os::raw::c_void) {
    let refcount = arg1 as *mut VecSliceRefCount;
    (*refcount).count += 1;
}

// Decrement a vec_slice ref count and possibly destroy it.
unsafe extern "C" fn vec_slice_unref(arg1: *mut ::std::os::raw::c_void) {
    let refcount = arg1 as *mut VecSliceRefCount;
    (*refcount).count -= 1;
    if (*refcount).count == 0 {
        // Recreate the `Box`s we used to create the slice and refcount objects.
        // Dropping them causes their data to be dropped, including the `Vec`
        // that was originally used to create the slice.
        let refcount = Box::from_raw(refcount);
        let slice = Box::from_raw(refcount.slice);
        mem::drop(refcount);
        mem::drop(slice);
    }
}

/// `MessageReader` is a zero-copy reader for the message payload.
///
/// To achieve zero-copy, use the BufRead API `fill_buf` and `consume`
/// to operate the reader.
pub struct MessageReader {
    reader: GrpcByteBufferReader,
    buffer_slice: GrpcSlice,
    buffer_offset: usize,
    remaining: usize,
}

impl MessageReader {
    /// Create a new `MessageReader`.
    ///
    /// Safety: `raw` must be a unique reference. The returned `MessageReader`
    /// has ownership of `raw` and will destroy `raw`. The caller should not
    /// keep a reference to `raw` or destroy it.
    pub unsafe fn new(raw: *mut grpc_byte_buffer) -> MessageReader {
        let reader = GrpcByteBufferReader::new(raw);
        let remaining = reader.len();

        MessageReader {
            reader,
            buffer_slice: Default::default(),
            buffer_offset: 0,
            remaining,
        }
    }
}

// These impls are safe because we ensure we have a unique reference to the
// underlying data.
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
        if self.remaining == 0 {
            return Ok(0);
        }
        buf.reserve(self.remaining);
        let start = buf.len();
        let mut len = start;
        unsafe {
            buf.set_len(start + self.remaining);
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
        if self.remaining == 0 {
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
        self.remaining -= amt;
        self.buffer_offset += amt;
    }
}

#[cfg(feature = "prost-codec")]
impl Buf for MessageReader {
    fn remaining(&self) -> usize {
        self.remaining
    }

    fn bytes(&self) -> &[u8] {
        // This is similar but not identical to `BuffRead::fill_buf`, since `self`
        // is not mutable, we can only return bytes up to the end of the current
        // slice.

        // Optimization for empty slice
        if self.buffer_slice.len() == 0 {
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
            if self.remaining == 0 {
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

/// A zero-copy writer.
///
/// This is implemented by writing into a Rust `Vec`, then wrapping that with the
/// necessary gRPC data structures (see `vec_slice`).
pub struct MessageWriter {
    pub write_buffer: Vec<u8>,
}

impl MessageWriter {
    /// Create an empty MessageWriter.
    pub fn new() -> MessageWriter {
        MessageWriter {
            write_buffer: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.write_buffer.clear();
    }

    /// Allocates `size` bytes for writing to and returns a pointer to the start
    /// of the newly allocated memory for writing into.
    pub fn reserve(&mut self, size: usize) -> &mut [u8] {
        let old_len = self.write_buffer.len();
        let new_len = old_len + size;
        self.write_buffer.reserve(size);
        unsafe {
            self.write_buffer.set_len(new_len);
            &mut self.write_buffer[old_len..]
        }
    }

    /// Safety: the caller takes responsibility for destroying the returned
    /// byte_buffer. Clears the internal buffer.
    pub unsafe fn byte_buffer(&mut self) -> *mut grpc_byte_buffer {
        let mut vec = Vec::new();
        mem::swap(&mut self.write_buffer, &mut vec);
        let slice = vec_slice(vec);
        grpc_sys::grpc_raw_byte_buffer_create(Box::into_raw(slice), 1)
    }
}

/// A wrapper for `MessageWriter` for implementing `Bytes::BufMut`. A wrapper is
/// needed because `BufMut` can be read and written incrementally, which
/// `MessageWriter` does not support.
///
/// Create a `MessageWriterBuf` by using `into` on a `MessageWriter`.
#[cfg(feature = "prost-codec")]
pub struct MessageWriterBuf<'a> {
    inner: &'a mut MessageWriter,
    offset: usize,
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
        self.inner.write_buffer.len() - self.offset
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.offset += cnt;
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inner.write_buffer[self.offset..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MessageWriter {
        fn len(&self) -> usize {
            self.write_buffer.len()
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
        assert_eq!(reader.remaining, data.len());
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

                assert_eq!(0, reader.remaining);
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
        assert_eq!(writer.len(), 3);
        writer.clear();
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn msg_writer_multi_write() {
        let mut writer = MessageWriter::new();
        assert_eq!(writer.len(), 0);
        let bytes = writer.reserve(3);
        bytes[0] = 42;
        let bytes = writer.reserve(3);
        bytes[2] = 255;
        let bytes = writer.reserve(2);
        bytes[1] = 0;
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
