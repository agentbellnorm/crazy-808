#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use engine::engine::Engine;
use engine::state::State;

use prost::Message;
use serde_json::to_string;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State as tState};

#[derive(Default)]
pub struct Roland808(Arc<Mutex<State>>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_event(
    event_name: &str,
    data: &str,
    state: tState<'_, Roland808>,
    engine: tState<'_, Engine>,
    app_handle: tauri::AppHandle,
) {
    println!("received event: {} with data: {}", event_name, data);

    match event_name {
        "variation-changed" => {
            engine.set_variation(data.to_string());
        }
        "start-stop" => {
            engine.toggle_playing();
        }
        "instrument-selected" => {
            engine.set_selected_instrument(data.parse().unwrap());
        }
        "channel-pressed" => {
            engine.toggle_channel(data.parse().unwrap());
        }
        "get-state" => (), // dont do anything, just let the state be published
        _ => panic!("what event! {}", event_name),
    }

    let state_handle = state.0.lock().unwrap();
    rs2js(serialize_state(&state_handle), &app_handle);
}

fn main() {
    let shared_state = Arc::new(Mutex::new(State::initial()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::sync_channel(1);

    let engine = Engine::new(shared_state.clone(), engine_output_sender);
    engine.run();

    let state_arc = shared_state.clone();

    tauri::Builder::default()
        .setup(|app| {
            rs2js(serialize_state(&State::initial()), &app.handle());
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
        .manage(engine)
        .manage(Roland808(shared_state))
        .invoke_handler(tauri::generate_handler![handle_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// A function that sends a message from Rust to JavaScript via a Tauri Event
fn rs2js<R: tauri::Runtime>(message: Vec<u8>, manager: &impl Manager<R>) {
    manager.emit_all("rs2js", message).unwrap();
}

pub fn serialize_state(state: &State) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(state.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    state.encode(&mut buf).unwrap();
    buf
}
