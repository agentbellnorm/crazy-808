extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/state.proto"], &["src"]).unwrap();
    tauri_build::build()
}
