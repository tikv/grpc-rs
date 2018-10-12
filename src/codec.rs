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

use call::MessageWriter;
use error::Result;

pub type DeserializeFn<T> = fn(&[u8]) -> Result<T>;
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
    use protobuf::{self, Message};

    use call::MessageWriter;
    use error::Result;

    #[inline]
    pub fn ser<T: Message>(t: &T, writer: &mut MessageWriter) {
        t.write_to_writer(writer).unwrap()
    }

    #[inline]
    pub fn de<T: Message>(buf: &[u8]) -> Result<T> {
        protobuf::parse_from_bytes(buf).map_err(From::from)
    }
}
