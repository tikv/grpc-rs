// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use grpcio_sys::*;
use std::cell::UnsafeCell;
use std::ffi::{c_void, CStr, CString};
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead, Read};
use std::mem::{self, ManuallyDrop, MaybeUninit};

/// Copied from grpc-sys/grpc/include/grpc/impl/codegen/slice.h. Unfortunately bindgen doesn't
/// generate it automatically.
const INLINED_SIZE: usize = mem::size_of::<libc::size_t>() + mem::size_of::<*mut u8>() - 1
    + mem::size_of::<*mut libc::c_void>();

/// A convenient rust wrapper for the type `grpc_slice`.
///
/// It's expected that the slice should be initialized.
#[repr(transparent)]
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

    /// Creates a slice from static rust slice.
    ///
    /// Same as `From<&[u8]>` but without copying the buffer.
    #[inline]
    pub fn from_static_slice(s: &'static [u8]) -> GrpcSlice {
        GrpcSlice(unsafe { grpc_slice_from_static_buffer(s.as_ptr() as _, s.len()) })
    }

    /// Creates a `GrpcSlice` from static rust str.
    ///
    /// Same as `from_str` but without allocation.
    #[inline]
    pub fn from_static_str(s: &'static str) -> GrpcSlice {
        GrpcSlice::from_static_slice(s.as_bytes())
    }

    /// Checks whether the slice stores bytes inline.
    pub fn is_inline(&self) -> bool {
        self.0.refcount.is_null()
    }

    /// Reallocates current slice with given capacity.
    ///
    /// The length of returned slice is the exact same as given cap.
    ///
    /// ## Safety
    ///
    /// Caller is expected to initialize all available bytes to guarantee safety of this slice.
    pub unsafe fn realloc(&mut self, cap: usize) -> &mut [MaybeUninit<u8>] {
        if cap <= INLINED_SIZE {
            // Only inlined slice can be reused safely.
            if !self.0.refcount.is_null() {
                *self = GrpcSlice::default();
            }
            self.0.data.inlined.length = cap as u8;
            std::slice::from_raw_parts_mut(
                self.0.data.inlined.bytes.as_mut_ptr() as *mut MaybeUninit<u8>,
                cap,
            )
        } else {
            *self = GrpcSlice(grpcio_sys::grpc_slice_malloc_large(cap));
            let start = self.0.data.refcounted.bytes;
            let len = self.0.data.refcounted.length;
            std::slice::from_raw_parts_mut(start as *mut MaybeUninit<u8>, len)
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut grpc_slice {
        &mut self.0
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

unsafe impl Send for GrpcSlice {}
unsafe impl Sync for GrpcSlice {}

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

unsafe extern "C" fn drop_vec(ptr: *mut c_void, len: usize) {
    Vec::from_raw_parts(ptr as *mut u8, len, len);
}

impl From<Vec<u8>> for GrpcSlice {
    /// Converts a `Vec<u8>` into `GrpcSlice`.
    ///
    /// If v can't fit inline, there will be allocations.
    #[inline]
    fn from(mut v: Vec<u8>) -> GrpcSlice {
        if v.is_empty() {
            return GrpcSlice::default();
        }

        if v.len() == v.capacity() {
            let slice = unsafe {
                grpcio_sys::grpc_slice_new_with_len(v.as_mut_ptr() as _, v.len(), Some(drop_vec))
            };
            mem::forget(v);
            return GrpcSlice(slice);
        }

        unsafe {
            GrpcSlice(grpcio_sys::grpc_slice_from_copied_buffer(
                v.as_mut_ptr() as _,
                v.len(),
            ))
        }
    }
}

/// Creates a `GrpcSlice` from rust string.
///
/// If the string can't fit inline, there will be allocations.
impl From<String> for GrpcSlice {
    #[inline]
    fn from(s: String) -> GrpcSlice {
        GrpcSlice::from(s.into_bytes())
    }
}

/// Creates a `GrpcSlice` from rust cstring.
///
/// If the cstring can't fit inline, there will be allocations.
impl From<CString> for GrpcSlice {
    #[inline]
    fn from(s: CString) -> GrpcSlice {
        GrpcSlice::from(s.into_bytes())
    }
}

/// Creates a `GrpcSlice` from rust slice.
///
/// The data inside slice will be cloned. If the data can't fit inline,
/// necessary buffer will be allocated.
impl From<&'_ [u8]> for GrpcSlice {
    #[inline]
    fn from(s: &'_ [u8]) -> GrpcSlice {
        GrpcSlice(unsafe { grpc_slice_from_copied_buffer(s.as_ptr() as _, s.len()) })
    }
}

/// Creates a `GrpcSlice` from rust str.
///
/// The data inside str will be cloned. If the data can't fit inline,
/// necessary buffer will be allocated.
impl From<&'_ str> for GrpcSlice {
    #[inline]
    fn from(s: &'_ str) -> GrpcSlice {
        GrpcSlice::from(s.as_bytes())
    }
}

/// Creates a `GrpcSlice` from rust `CStr`.
///
/// The data inside `CStr` will be cloned. If the data can't fit inline,
/// necessary buffer will be allocated.
impl From<&'_ CStr> for GrpcSlice {
    #[inline]
    fn from(s: &'_ CStr) -> GrpcSlice {
        GrpcSlice::from(s.to_bytes())
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

    fn chunk(&self) -> &[u8] {
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

                assert_eq!(dest[..amt], expect[..amt], "len: {len}, nslice: {n_slice}");

                // Read after move.
                let mut box_reader = Box::new(reader);
                let amt = box_reader.read(&mut dest).unwrap();
                assert_eq!(dest[..amt], expect[..amt], "len: {len}, nslice: {n_slice}");

                // Test read_to_end.
                let mut reader = new_message_reader(source.clone(), n_slice);
                let mut dest = vec![];
                reader.read_to_end(&mut dest).unwrap();
                assert_eq!(dest, expect, "len: {len}, nslice: {n_slice}");

                assert_eq!(0, reader.len());
                assert_eq!(0, reader.read(&mut [1]).unwrap());

                // Test arbitrary consuming.
                let mut reader = new_message_reader(source.clone(), n_slice);
                reader.consume(source.len() * (n_slice - 1));
                let mut dest = vec![];
                reader.read_to_end(&mut dest).unwrap();
                assert_eq!(dest.len(), source.len(), "len: {len}, nslice: {n_slice}");
                assert_eq!(
                    *dest,
                    expect[expect.len() - source.len()..],
                    "len: {len}, nslice: {n_slice}"
                );
                assert_eq!(0, reader.len());
                assert_eq!(0, reader.read(&mut [1]).unwrap());
            }
        }
    }

    #[test]
    fn test_converter() {
        let a = vec![1, 2, 3, 0];
        assert_eq!(GrpcSlice::from(a.clone()).as_slice(), a.as_slice());
        assert_eq!(GrpcSlice::from(a.as_slice()).as_slice(), a.as_slice());

        let s = "abcd".to_owned();
        assert_eq!(GrpcSlice::from(s.clone()).as_slice(), s.as_bytes());
        assert_eq!(GrpcSlice::from(s.as_str()).as_slice(), s.as_bytes());

        let cs = CString::new(s.clone()).unwrap();
        assert_eq!(GrpcSlice::from(cs.clone()).as_slice(), s.as_bytes());
        assert_eq!(GrpcSlice::from(cs.as_c_str()).as_slice(), s.as_bytes());
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
                    let bytes = Buf::chunk(&reader);
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
