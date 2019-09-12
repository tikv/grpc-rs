// Copyright 2017 PingCAP, Inc.
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

use crate::call::{MessageReader, MessageWriter};
use crate::error::Result;

pub type DeserializeFn<T> = fn(MessageReader) -> Result<T>;
pub type SerializeFn<T> = fn(&T, &mut MessageWriter);

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
    use protobuf::stream::CodedOutputStream;
    use protobuf::{CodedInputStream, Message};

    use super::{MessageReader, MessageWriter};
    use crate::error::Result;

    #[inline]
    pub fn ser<T: Message>(t: &T, writer: &mut MessageWriter) {
        let size = t.compute_size();
        writer.reserve(size as usize);
        let mut cos = CodedOutputStream::new(writer);
        t.check_initialized().unwrap();
        t.write_to_with_cached_sizes(&mut cos).unwrap();
        cos.flush().unwrap();
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

    use crate::error::Result;
    use crate::{MessageReader, MessageWriter};

    #[inline]
    pub fn ser<M: Message>(m: &M, writer: &mut MessageWriter) {
        use std::io::Write;
        writer.reserve(m.encoded_len());
        // Because we've already got a reserved writer, we don't need length checks.
        m.encode_raw(writer);
        writer.flush().expect("Writing message to buffer failed");
    }

    #[inline]
    pub fn de<M: Message + Default>(mut reader: MessageReader) -> Result<M> {
        use bytes::buf::Buf;
        reader.advance(0);
        M::decode(reader).map_err(Into::into)
    }
}
