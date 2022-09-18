#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod sound;
mod engine;

use std::sync::{Mutex, Arc};
use serde_json::to_string;
use tauri::{State, Manager};
use engine::Engine;
use crate::engine::MachineState;

#[derive(Default)]
pub struct Roland808(Arc<Mutex<MachineState>>);

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_event(event_name: &str, data: &str, state: State<'_, Roland808>) {

    println!("received event: {} with data: {}", event_name, data);

    let mut state_handle = state.0.lock().unwrap();

    match event_name {
        "variation-changed" => {
            state_handle.current_variation = data.to_string();
        },
        "start-stop" => {
            state_handle.playing = !state_handle.playing;
        }
        _ => panic!("what event! {}", event_name)
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new(MachineState::default()));
    let (engine_output_sender, engine_output_receiver) = std::sync::mpsc::channel();

    let engine = Engine::new(shared_state.clone(), engine_output_sender);
    engine.run();

    let state_arc = shared_state.clone();
    
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            std::thread::spawn(move || {
                loop {
                    if engine_output_receiver.recv().is_ok() {
                        let raw = &*state_arc.lock().unwrap();
                        let serialized = to_string(raw).unwrap();
                        rs2js(serialized, &app_handle);
                    }
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
fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    manager
        .emit_all("rs2js", message)
        .unwrap();
}
