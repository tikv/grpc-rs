#![feature(test)]

extern crate grpcio_sys as grpc_sys;
extern crate test;

use grpc_sys::GrpcwrapTag;
use test::Bencher;

enum MockTag {
    // Size of CallTag is roughly 40 bytes.
    Batch([u8; 40]),
    Request([u8; 40]),
    UnaryRequest([u8; 40]),
    Abort([u8; 40]),
    Shutdown([u8; 40]),
    Spawn([u8; 40]),
}

impl MockTag {
    /// Consumes the `MockTag`, returning the wrapped raw pointer.
    pub fn into_raw(self) -> *mut GrpcwrapTag {
        if let MockTag::Spawn(_) = self {
            panic!("MockTag::Spawn can not into raw pointer")
        }
        let tag_box = Box::new(self);
        let tag_ptr = Box::into_raw(tag_box);
        unsafe {
            grpc_sys::grpcwrap_tag_wrap(tag_ptr as _)
        }
    }

    /// Constructs a `MockTag` from a raw pointer.
    pub unsafe fn from_raw(tag_ptr: *mut GrpcwrapTag) -> Box<MockTag> {
        let ptr = grpc_sys::grpcwrap_tag_unwrap(tag_ptr as _);
        let tag_box = Box::from_raw(ptr as _);
        match *tag_box {
            // Spawn is notified from Alarm, Alarm manages the `tag_ptr` lifetime.
            MockTag::Spawn(_) => (),
            _ => {
                grpc_sys::grpcwrap_tag_destroy(tag_ptr as _);
            }
        }
        tag_box
    }
}

#[bench]
fn bench_tag_conversion(b: &mut Bencher) {
    b.iter(|| {
        let tag = MockTag::Request([7; 40]);
        let tag_ptr = tag.into_raw();
        test::black_box(unsafe {
            MockTag::from_raw(tag_ptr)
        });
    })
}
