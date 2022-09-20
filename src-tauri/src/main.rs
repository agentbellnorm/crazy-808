#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod engine;
mod sound;

// generated from state.proto. Find with: find src-tauri/target -name "crazy.state.rs"
pub mod state {
    include!(concat!(env!("OUT_DIR"), "/crazy.state.rs"));
}

use engine::Engine;
use serde_json::to_string;
use state::State;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State as tState};
use std::io::Cursor;
use prost::Message;

#[derive(Default)]
pub struct Roland808(Arc<Mutex<State>>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_event(event_name: &str, data: &str, state: tState<'_, Roland808>) {
    println!("received event: {} with data: {}", event_name, data);

    let mut state_handle = state.0.lock().unwrap();

    match event_name {
        "variation-changed" => {
            state_handle.current_variation = data.to_string();
        }
        "start-stop" => {
            state_handle.playing = !state_handle.playing;
        }
        "instrument-selected" => {
            state_handle.selected_instrument = data.to_string();
        }
        _ => panic!("what event! {}", event_name),
    }
}

fn main() {
    println!("{}", env!("OUT_DIR").to_string());
    let shared_state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::channel();

    let engine = Engine::new(shared_state.clone(), engine_output_sender);
    engine.run();

    let state_arc = shared_state.clone();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            std::thread::spawn(move || loop {
                if engine_output_receiver.recv().is_ok() {
                    let raw = &*state_arc.lock().unwrap();
                    let serialized = serialize_state(raw);
                    rs2js(serialized, &app_handle);
                }
            });
            Ok(())
        })
        .manage(Roland808(shared_state))
        .invoke_handler(tauri::generate_handler![handle_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// A function that sends a message from Rust to JavaScript via a Tauri Event
fn rs2js<R: tauri::Runtime>(message: Vec<u8>, manager: &impl Manager<R>) {
    manager.emit_all("rs2js", message).unwrap();
}

pub fn serialize_state(state: &state::State) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(state.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    state.encode(&mut buf).unwrap();
    buf
}