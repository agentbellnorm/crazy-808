pub mod engine;
mod sound;

// generated from state.proto. Find with: find web-gui-tauri/target -name "crazy.state.rs"
pub mod state {
    include!(concat!(env!("OUT_DIR"), "/crazy.state.rs"));
}
