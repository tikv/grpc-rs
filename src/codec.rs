// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::buf::GrpcSlice;
use crate::call::MessageReader;
use crate::error::Result;

pub type DeserializeFn<T> = fn(MessageReader) -> Result<T>;
pub type SerializeFn<T> = fn(&T, &mut GrpcSlice);

/// Defines how to serialize and deserialize between the specialized type and byte slice.
pub struct Marshaller<T> {
    // Use function pointer here to simplify the signature.
    // Compiler will probably inline the function so performance
    // impact can be omitted.
    //
    // Using trait will require a trait object or generic, which will
    // either have performance impact or make signature complicated.
    //
    // const function is not stable yet (rust-lang/rust#24111), hence
    // make all fields public.
    /// The serialize function.
    pub ser: SerializeFn<T>,

    /// The deserialize function.
    pub de: DeserializeFn<T>,
}

#[cfg(feature = "protobuf-codec")]
pub mod pb_codec {
    use protobuf::{CodedInputStream, CodedOutputStream, Message};

    use super::MessageReader;
    use crate::buf::GrpcSlice;
    use crate::error::Result;

    #[inline]
    pub fn ser<T: Message>(t: &T, buf: &mut GrpcSlice) {
        let cap = t.compute_size();
        unsafe {
            let bytes = buf.realloc(cap as usize);
            let raw_bytes = &mut *(bytes as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
            let mut s = CodedOutputStream::bytes(raw_bytes);
            t.write_to_with_cached_sizes(&mut s).unwrap();
        }
    }

    #[inline]
    pub fn de<T: Message>(mut reader: MessageReader) -> Result<T> {
        let mut s = CodedInputStream::from_buffered_reader(&mut reader);
        let mut m = T::new();
        m.merge_from(&mut s)?;
        Ok(m)
    }
}

#[cfg(feature = "prost-codec")]
pub mod pr_codec {
    use prost::Message;

    use super::MessageReader;
    use crate::buf::GrpcSlice;
    use crate::error::Result;

    #[inline]
    pub fn ser<M: Message>(msg: &M, buf: &mut GrpcSlice) {
        let size = msg.encoded_len();
        unsafe {
            let bytes = buf.realloc(size);
            let mut b = &mut *(bytes as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
            msg.encode(&mut b)
                .expect("Writing message to buffer failed");
            debug_assert!(b.is_empty());
        }
    }

    #[inline]
    pub fn de<M: Message + Default>(mut reader: MessageReader) -> Result<M> {
        use bytes::buf::Buf;
        reader.advance(0);
        M::decode(reader).map_err(Into::into)
    }
}
