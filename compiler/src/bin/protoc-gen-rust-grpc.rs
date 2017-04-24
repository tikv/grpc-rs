extern crate compiler;

use compiler::codegen;

fn main() {
    codegen::protoc_gen_grpc_rust_main();
}
