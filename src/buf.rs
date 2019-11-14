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

use grpcio_sys::*;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead, Read};
use std::mem::{self, ManuallyDrop, MaybeUninit};
use std::ptr;
use std::sync::atomic::{self, AtomicUsize, Ordering};

/// A convenient rust wrapper for the type `grpc_slice`.
///
/// It's expected that the slice should be initialized.
#[repr(C)]
pub struct GrpcSlice(grpc_slice);

impl GrpcSlice {
    /// Get the length of the data.
    pub fn len(&self) -> usize {
        unsafe {
            if !self.0.refcount.is_null() {
                self.0.data.refcounted.length
            } else {
                self.0.data.inlined.length as usize
            }
        }
    }

    /// Returns a slice of inner buffer.
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            if !self.0.refcount.is_null() {
                let start = self.0.data.refcounted.bytes;
                let len = self.0.data.refcounted.length;
                std::slice::from_raw_parts(start, len)
            } else {
                let len = self.0.data.inlined.length;
                &self.0.data.inlined.bytes[..len as usize]
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Clone for GrpcSlice {
    /// Clone the slice.
    ///
    /// If the slice is not inlined, the reference count will be increased
    /// instead of copy.
    fn clone(&self) -> Self {
        GrpcSlice(unsafe { grpc_slice_ref(self.0) })
    }
}

impl Default for GrpcSlice {
    /// Returns a default slice, which is empty.
    fn default() -> Self {
        GrpcSlice(unsafe { grpc_empty_slice() })
    }
}

impl Debug for GrpcSlice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl Drop for GrpcSlice {
    fn drop(&mut self) {
        unsafe {
            grpc_slice_unref(self.0);
        }
    }
}

impl PartialEq<[u8]> for GrpcSlice {
    fn eq(&self, r: &[u8]) -> bool {
        // Technically, the equal function inside vtable should be used.
        // But it's not cheap or safe to create a grpc_slice from rust slice.
        self.as_slice() == r
    }
}

impl PartialEq<GrpcSlice> for GrpcSlice {
    fn eq(&self, r: &GrpcSlice) -> bool {
        unsafe { grpc_slice_eq(self.0, r.0) != 0 }
    }
}

/// `grpc_slice` use `grpc_slice_refcount` to trace the lifetime of inner data.
///
/// When ref count decreases to 0, there will be no access to the data.
/// To utilize the mechanism, we need to define a struct which has
/// `grpc_slice_refcount` type as the first field, so that it's safe to cast
/// a pointer to the struct to `grpc_slice_refcount` and all ref/unref operations
/// will be forwarded to our own implement.
///
/// Vec will be stored inside the struct so that it will be dropped automatically
/// once the struct is dropped.
///
/// Note that the struct should not be moved if `sub_refcount` at `grpc_slice_refcount`
/// points back to the `VecRefCount` itself.
#[repr(C)]
struct VecRefCount {
    rc: grpc_slice_refcount,
    refs: AtomicUsize,
    v: Vec<u8>,
}

unsafe extern "C" fn vec_ref(rc_ptr: *mut c_void) {
    let rc_ptr = rc_ptr as *mut VecRefCount;
    (*rc_ptr).refs.fetch_add(1, Ordering::Relaxed);
}

unsafe extern "C" fn vec_unref(rc_ptr: *mut c_void) {
    let rc_ptr = rc_ptr as *mut VecRefCount;
    if (*rc_ptr).refs.fetch_sub(1, Ordering::Release) != 1 {
        return;
    }

    atomic::fence(Ordering::Acquire);
    Box::from_raw(rc_ptr);
}

/// The global vtable for vec.
const VEC_REF_COUNT_VTABLE: grpc_slice_refcount_vtable = grpc_slice_refcount_vtable {
    ref_: Some(vec_ref),
    unref: Some(vec_unref),
    eq: Some(grpc_slice_default_eq_impl),
    hash: Some(grpc_slice_default_hash_impl),
};

impl From<Vec<u8>> for GrpcSlice {
    /// Converts a `Vec<u8>` into `GrpcSlice`.
    ///
    /// There will be a small allocation for custom vtable if the data length
    /// is too large (23 bytes currently).
    #[inline]
    fn from(mut v: Vec<u8>) -> GrpcSlice {
        let mut slice = GrpcSlice::default();
        if v.is_empty() {
            return slice;
        } else if v.len() <= mem::size_of_val(unsafe { &slice.0.data.inlined.bytes }) {
            unsafe {
                slice.0.data.inlined.length = v.len() as u8;
                slice.0.data.inlined.bytes[..v.len()].copy_from_slice(&v);
            }
            return slice;
        }
        unsafe {
            slice.0.data = grpc_slice_grpc_slice_data {
                refcounted: grpc_slice_grpc_slice_data_grpc_slice_refcounted {
                    bytes: v.as_mut_ptr(),
                    length: v.len(),
                },
            };

            let mut ref_count = Box::new(VecRefCount {
                rc: grpc_slice_refcount {
                    vtable: &VEC_REF_COUNT_VTABLE,
                    sub_refcount: ptr::null_mut(),
                },
                refs: AtomicUsize::new(1),
                v,
            });
            ref_count.rc.sub_refcount = &mut ref_count.rc;
            slice.0.refcount = Box::into_raw(mem::transmute(ref_count));
            slice
        }
    }
}

/// A collection of `GrpcBytes`.
#[repr(C)]
pub struct GrpcByteBuffer(*mut grpc_byte_buffer);

impl GrpcByteBuffer {
    #[inline]
    pub unsafe fn from_raw(ptr: *mut grpc_byte_buffer) -> GrpcByteBuffer {
        GrpcByteBuffer(ptr)
    }
}

impl<'a> From<&'a [GrpcSlice]> for GrpcByteBuffer {
    /// Create a buffer from the given slice array.
    ///
    /// A buffer is allocated for the whole slice array, and every slice will
    /// be `Clone::clone` into the buffer.
    fn from(slice: &'a [GrpcSlice]) -> Self {
        let len = slice.len();
        unsafe {
            let s = slice.as_ptr() as *const grpc_slice as *const UnsafeCell<grpc_slice>;
            // hack: see From<&GrpcSlice>.
            GrpcByteBuffer(grpc_raw_byte_buffer_create((*s).get(), len))
        }
    }
}

impl<'a> From<&'a GrpcSlice> for GrpcByteBuffer {
    /// Create a buffer from the given single slice.
    ///
    /// A buffer, which length is 1, is allocated for the slice.
    #[allow(clippy::cast_ref_to_mut)]
    fn from(s: &'a GrpcSlice) -> GrpcByteBuffer {
        unsafe {
            // hack: buffer_create accepts an mutable pointer to indicate it mutate
            // ref count. Ref count is recorded by atomic variable, which is considered
            // `Sync` in rust. This is an interesting difference in what is *mutable*
            // between C++ and rust.
            // Using `UnsafeCell` to avoid raw cast, which is UB.
            let s = &*(s as *const GrpcSlice as *const grpc_slice as *const UnsafeCell<grpc_slice>);
            GrpcByteBuffer(grpc_raw_byte_buffer_create((*s).get(), 1))
        }
    }
}

impl Clone for GrpcByteBuffer {
    fn clone(&self) -> Self {
        unsafe { GrpcByteBuffer(grpc_byte_buffer_copy(self.0)) }
    }
}

impl Drop for GrpcByteBuffer {
    fn drop(&mut self) {
        unsafe { grpc_byte_buffer_destroy(self.0) }
    }
}

/// A zero-copy reader for the message payload.
///
/// To achieve zero-copy, use the BufRead API `fill_buf` and `consume`
/// to operate the reader.
#[repr(C)]
pub struct GrpcByteBufferReader {
    reader: grpc_byte_buffer_reader,
    /// Current reading buffer.
    // This is a temporary buffer that may need to be dropped before every
    // iteration. So use `ManuallyDrop` to control the behavior more clean
    // and precisely.
    slice: ManuallyDrop<GrpcSlice>,
    /// The offset of `slice` that has not been read.
    offset: usize,
    /// How many bytes pending for reading.
    remain: usize,
}

impl GrpcByteBufferReader {
    /// Creates a reader for the `GrpcByteBuffer`.
    ///
    /// `buf` is stored inside the reader, and dropped when the reader is dropped.
    pub fn new(buf: GrpcByteBuffer) -> GrpcByteBufferReader {
        let mut reader = MaybeUninit::uninit();
        let mut s = MaybeUninit::uninit();
        unsafe {
            let code = grpc_byte_buffer_reader_init(reader.as_mut_ptr(), buf.0);
            assert_eq!(code, 1);
            if 0 == grpc_byte_buffer_reader_next(reader.as_mut_ptr(), s.as_mut_ptr()) {
                s.as_mut_ptr().write(grpc_empty_slice());
            }
            let remain = grpc_byte_buffer_length((*reader.as_mut_ptr()).buffer_out);
            // buf is stored inside `reader` as `buffer_in`, so do not drop it.
            mem::forget(buf);

            GrpcByteBufferReader {
                reader: reader.assume_init(),
                slice: ManuallyDrop::new(GrpcSlice(s.assume_init())),
                offset: 0,
                remain,
            }
        }
    }

    /// Get the next slice from reader.
    fn load_next_slice(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.slice);
            if 0 == grpc_byte_buffer_reader_next(&mut self.reader, &mut self.slice.0) {
                self.slice = ManuallyDrop::new(GrpcSlice::default());
            }
        }
        self.offset = 0;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.remain
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.remain == 0
    }
}

impl Read for GrpcByteBufferReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.fill_buf()?.read(buf)?;
        self.consume(read);
        Ok(read)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let cap = self.remain;
        buf.reserve(cap);
        let old_len = buf.len();
        while self.remain > 0 {
            let read = {
                let s = match self.fill_buf() {
                    Ok(s) => s,
                    Err(e) => {
                        unsafe {
                            buf.set_len(old_len);
                        }
                        return Err(e);
                    }
                };
                buf.extend_from_slice(s);
                s.len()
            };
            self.consume(read);
        }
        Ok(cap)
    }
}

impl BufRead for GrpcByteBufferReader {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.slice.is_empty() {
            return Ok(&[]);
        }
        Ok(unsafe { self.slice.as_slice().get_unchecked(self.offset..) })
    }

    fn consume(&mut self, mut amt: usize) {
        if amt > self.remain {
            amt = self.remain;
        }
        self.remain -= amt;
        let mut offset = self.offset + amt;
        while offset >= self.slice.len() && offset > 0 {
            offset -= self.slice.len();
            self.load_next_slice();
        }
        self.offset = offset;
    }
}

impl Drop for GrpcByteBufferReader {
    fn drop(&mut self) {
        unsafe {
            grpc_byte_buffer_reader_destroy(&mut self.reader);
            ManuallyDrop::drop(&mut self.slice);
            grpc_byte_buffer_destroy(self.reader.buffer_in);
        }
    }
}

unsafe impl Sync for GrpcByteBufferReader {}
unsafe impl Send for GrpcByteBufferReader {}

#[cfg(feature = "prost-codec")]
impl bytes::Buf for GrpcByteBufferReader {
    fn remaining(&self) -> usize {
        self.remain
    }

    fn bytes(&self) -> &[u8] {
        // This is similar but not identical to `BuffRead::fill_buf`, since `self`
        // is not mutable, we can only return bytes up to the end of the current
        // slice.

        // Optimization for empty slice
        if self.slice.is_empty() {
            return &[];
        }

        unsafe { self.slice.as_slice().get_unchecked(self.offset..) }
    }

    fn advance(&mut self, cnt: usize) {
        self.consume(cnt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_message_reader(seed: Vec<u8>, copy_count: usize) -> GrpcByteBufferReader {
        let data = vec![GrpcSlice::from(seed); copy_count];
        let buf = GrpcByteBuffer::from(data.as_slice());
        GrpcByteBufferReader::new(buf)
    }

    #[test]
    fn test_grpc_slice() {
        let empty = GrpcSlice::default();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(empty.as_slice().is_empty());

        let a = vec![0, 2, 1, 3, 8];
        let slice = GrpcSlice::from(a.clone());
        assert_eq!(a.as_slice(), slice.as_slice());
        assert_eq!(a.len(), slice.len());
        assert_eq!(&slice, &*a);

        let a = vec![5; 64];
        let slice = GrpcSlice::from(a.clone());
        assert_eq!(a.as_slice(), slice.as_slice());
        assert_eq!(a.len(), slice.len());
        assert_eq!(&slice, &*a);

        let a = vec![];
        let slice = GrpcSlice::from(a);
        assert_eq!(empty, slice);
    }

    #[test]
    // Old code crashes under a very weird circumstance, due to a typo in `MessageReader::consume`
    fn test_typo_len_offset() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // half of the size of `data`
        let half_size = data.len() / 2;
        let slice = GrpcSlice::from(data.clone());
        let buffer = GrpcByteBuffer::from(&slice);
        let mut reader = GrpcByteBufferReader::new(buffer);
        assert_eq!(reader.len(), data.len());
        // first 3 elements of `data`
        let mut buf = vec![0; half_size];
        reader.read(buf.as_mut_slice()).unwrap();
        assert_eq!(data[..half_size], *buf.as_slice());
        assert_eq!(reader.len(), data.len() - half_size);
        assert!(!reader.is_empty());
        reader.read(&mut buf).unwrap();
        assert_eq!(data[half_size..], *buf.as_slice());
        assert_eq!(reader.len(), 0);
        assert!(reader.is_empty());
    }

    #[test]
    fn test_message_reader() {
        for len in 0..=1024 {
            for n_slice in 1..=4 {
                let source = vec![len as u8; len];
                let expect = vec![len as u8; len * n_slice];
                // Test read.
                let mut reader = new_message_reader(source.clone(), n_slice);
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
                let mut reader = new_message_reader(source.clone(), n_slice);
                let mut dest = vec![];
                reader.read_to_end(&mut dest).unwrap();
                assert_eq!(dest, expect, "len: {}, nslice: {}", len, n_slice);

                assert_eq!(0, reader.len());
                assert_eq!(0, reader.read(&mut [1]).unwrap());

                // Test arbitrary consuming.
                let mut reader = new_message_reader(source.clone(), n_slice);
                reader.consume(source.len() * (n_slice - 1));
                let mut dest = vec![];
                reader.read_to_end(&mut dest).unwrap();
                assert_eq!(
                    dest.len(),
                    source.len(),
                    "len: {}, nslice: {}",
                    len,
                    n_slice
                );
                assert_eq!(
                    *dest,
                    expect[expect.len() - source.len()..],
                    "len: {}, nslice: {}",
                    len,
                    n_slice
                );
                assert_eq!(0, reader.len());
                assert_eq!(0, reader.read(&mut [1]).unwrap());
            }
        }
    }

    #[cfg(feature = "prost-codec")]
    #[test]
    fn test_buf_impl() {
        use bytes::Buf;

        for len in 0..1024 + 1 {
            for n_slice in 1..4 {
                let source = vec![len as u8; len];

                let mut reader = new_message_reader(source.clone(), n_slice);

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
}
