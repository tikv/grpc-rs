// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::buf::GrpcSlice;
use crate::call::MessageReader;
use crate::error::Result;

pub type DeserializeFn<T> = fn(MessageReader) -> Result<T>;
pub type SerializeFn<T> = fn(&T, &mut GrpcSlice) -> Result<()>;

/// According to <https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>, grpc uses
/// a four bytes to describe the length of a message, so it should not exceed u32::MAX.
pub const MAX_MESSAGE_SIZE: usize = u32::MAX as usize;

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

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub mod pb_codec {
    use std::fmt;
    use std::marker::PhantomData;

    #[cfg(feature = "protobuf-codec")]
    use protobuf::{CodedOutputStream, Message};

    #[cfg(feature = "protobufv3-codec")]
    use protobufv3::{CodedOutputStream, Message};

    use super::{from_buf_read, MessageReader, MAX_MESSAGE_SIZE};
    use crate::buf::GrpcSlice;
    use crate::error::{Error, Result};

    #[inline]
    fn encode_message<T: Message>(t: &T, buf: &mut GrpcSlice) -> Result<()> {
        let cap = t.compute_size() as usize;
        // FIXME: This is not a practical fix until stepancheg/rust-protobuf#530 is fixed.
        if cap <= MAX_MESSAGE_SIZE {
            unsafe {
                let bytes = buf.realloc(cap);
                let raw_bytes = &mut *(bytes as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
                let mut s = CodedOutputStream::bytes(raw_bytes);
                t.write_to_with_cached_sizes(&mut s).map_err(Into::into)
            }
        } else {
            Err(Error::Codec(
                format!("message is too large: {cap} > {MAX_MESSAGE_SIZE}").into(),
            ))
        }
    }

    #[inline]
    fn decode_message<T: Message>(mut reader: MessageReader) -> Result<T> {
        let mut s = from_buf_read(&mut reader);
        let mut m = T::new();
        m.merge_from(&mut s)?;
        Ok(m)
    }

    /// Trait used by the public protobuf marshaller to support raw messages as well as
    /// pre-encoded offload responses.
    #[doc(hidden)]
    pub trait PbMessageSerialize {
        fn serialize(&self, buf: &mut GrpcSlice) -> Result<()>;
    }

    impl<T: Message> PbMessageSerialize for T {
        #[inline]
        fn serialize(&self, buf: &mut GrpcSlice) -> Result<()> {
            encode_message(self, buf)
        }
    }

    /// Trait used by the public protobuf marshaller to support raw messages as well as
    /// deferred-decoding offload requests.
    #[doc(hidden)]
    pub trait PbMessageDeserialize: Sized {
        fn deserialize(reader: MessageReader) -> Result<Self>;
    }

    impl<T: Message> PbMessageDeserialize for T {
        #[inline]
        fn deserialize(reader: MessageReader) -> Result<Self> {
            decode_message(reader)
        }
    }

    /// Defers protobuf decoding until the request is explicitly consumed by application code.
    ///
    /// This is primarily useful for server handlers that want to move decode work onto a worker
    /// pool instead of doing it on a gRPC poll thread.
    #[cfg(feature = "offload-codec")]
    pub struct Req<T> {
        input: MessageReader,
        _req: PhantomData<T>,
    }

    #[cfg(feature = "offload-codec")]
    impl<T> Req<T> {
        /// Returns the size of the encoded protobuf payload in bytes.
        #[inline]
        pub fn encoded_len(&self) -> usize {
            self.input.len()
        }

        /// Returns the underlying gRPC message reader for custom decode flows.
        #[inline]
        pub fn into_reader(self) -> MessageReader {
            self.input
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T: Message> Req<T> {
        /// Decodes the wrapped protobuf request.
        #[inline]
        pub fn get(self) -> Result<T> {
            decode_message(self.input)
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T> fmt::Debug for Req<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Req")
                .field("encoded_len", &self.encoded_len())
                .field("message_type", &std::any::type_name::<T>())
                .finish()
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T: Message> PbMessageDeserialize for Req<T> {
        #[inline]
        fn deserialize(reader: MessageReader) -> Result<Self> {
            Ok(Self {
                input: reader,
                _req: PhantomData,
            })
        }
    }

    /// Stores a protobuf response that has already been serialized into a `GrpcSlice`.
    ///
    /// This lets application code encode on a worker pool and hand the final bytes back to gRPC
    /// without doing another serialization pass on the poll thread.
    #[cfg(feature = "offload-codec")]
    #[derive(Clone)]
    pub struct Resp<T> {
        output: GrpcSlice,
        _resp: PhantomData<T>,
    }

    #[cfg(feature = "offload-codec")]
    impl<T> Resp<T> {
        /// Returns the size of the encoded protobuf payload in bytes.
        #[inline]
        pub fn encoded_len(&self) -> usize {
            self.output.len()
        }

        /// Returns the serialized protobuf payload.
        #[inline]
        pub fn as_slice(&self) -> &[u8] {
            self.output.as_slice()
        }

        /// Extracts the serialized payload.
        #[inline]
        pub fn into_slice(self) -> GrpcSlice {
            self.output
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T: Message> Resp<T> {
        /// Serializes a protobuf response into a `GrpcSlice`.
        #[inline]
        pub fn new(message: T) -> Result<Self> {
            let mut output = GrpcSlice::default();
            encode_message(&message, &mut output)?;
            Ok(Self {
                output,
                _resp: PhantomData,
            })
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T> fmt::Debug for Resp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resp")
                .field("encoded_len", &self.encoded_len())
                .field("message_type", &std::any::type_name::<T>())
                .finish()
        }
    }

    #[cfg(feature = "offload-codec")]
    impl<T> PbMessageSerialize for Resp<T> {
        #[inline]
        fn serialize(&self, buf: &mut GrpcSlice) -> Result<()> {
            *buf = self.output.clone();
            Ok(())
        }
    }

    #[inline]
    pub fn ser<T: PbMessageSerialize>(t: &T, buf: &mut GrpcSlice) -> Result<()> {
        t.serialize(buf)
    }

    #[inline]
    pub fn de<T: PbMessageDeserialize>(reader: MessageReader) -> Result<T> {
        T::deserialize(reader)
    }
}

#[cfg(feature = "protobuf-codec")]
fn from_buf_read(reader: &mut MessageReader) -> protobuf::CodedInputStream {
    protobuf::CodedInputStream::from_buffered_reader(reader)
}

#[cfg(feature = "protobufv3-codec")]
fn from_buf_read(reader: &mut MessageReader) -> protobufv3::CodedInputStream {
    protobufv3::CodedInputStream::from_buf_read(reader)
}

#[cfg(feature = "prost-codec")]
pub mod pr_codec {
    use prost::Message;

    use super::{MessageReader, MAX_MESSAGE_SIZE};
    use crate::buf::GrpcSlice;
    use crate::error::{Error, Result};

    #[inline]
    pub fn ser<M: Message>(msg: &M, buf: &mut GrpcSlice) -> Result<()> {
        let size = msg.encoded_len();
        if size <= MAX_MESSAGE_SIZE {
            unsafe {
                let bytes = buf.realloc(size);
                let mut b = &mut *(bytes as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
                msg.encode(&mut b)?;
                debug_assert!(b.is_empty());
            }
            Ok(())
        } else {
            Err(Error::Codec(
                format!("message is too large: {size} > {MAX_MESSAGE_SIZE}").into(),
            ))
        }
    }

    #[inline]
    pub fn de<M: Message + Default>(mut reader: MessageReader) -> Result<M> {
        use bytes::buf::Buf;
        reader.advance(0);
        M::decode(reader).map_err(Into::into)
    }
}

#[cfg(all(test, feature = "protobuf-codec", feature = "offload-codec"))]
mod tests {
    use protobuf::well_known_types::wrappers::StringValue;

    use super::pb_codec::{de, ser, Req, Resp};
    use crate::buf::GrpcByteBuffer;
    use crate::call::MessageReader;
    use crate::GrpcSlice;

    fn string_value(value: &str) -> StringValue {
        let mut message = StringValue::new();
        message.value = value.to_owned();
        message
    }

    fn to_reader(slice: GrpcSlice) -> MessageReader {
        let buf = GrpcByteBuffer::from(&slice);
        MessageReader::new(buf)
    }

    #[test]
    fn test_pb_req_defers_decode() {
        let expected = string_value("offload");
        let mut payload = GrpcSlice::default();
        ser(&expected, &mut payload).unwrap();

        let req = de::<Req<StringValue>>(to_reader(payload)).unwrap();
        assert!(req.encoded_len() > 0);

        let decoded = req.get().unwrap();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_pb_resp_reuses_encoded_payload() {
        let message = string_value("offload");

        let mut expected = GrpcSlice::default();
        ser(&message, &mut expected).unwrap();

        let resp = Resp::new(message).unwrap();
        assert_eq!(resp.as_slice(), expected.as_slice());

        let mut actual = GrpcSlice::default();
        ser(&resp, &mut actual).unwrap();
        assert_eq!(actual, expected);
    }
}
