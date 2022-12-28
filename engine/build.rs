extern crate prost_build;
use protobuf_src::protoc;

fn main() {
    // bring in protoc via cargo so it gets compiled, rather than adding it to builder image.
    std::env::set_var("PROTOC", protoc());
    prost_build::compile_protos(&["src/state.proto"], &["src"]).unwrap();
}
