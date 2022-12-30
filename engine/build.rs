extern crate prost_build;

fn main() {
    // bring in protoc via cargo so it gets compiled, rather than adding it to builder image.
    prost_build::compile_protos(&["src/state.proto"], &["src"]).unwrap();
}
