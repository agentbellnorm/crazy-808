#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod sound;
mod engine;

use std::sync::{Mutex, Arc};
use tauri::State;
use engine::Engine;
use crate::engine::MachineState;

#[derive(Default)]
pub struct Roland808(Arc<Mutex<MachineState>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_event(event_name: &str, data: &str, state: State<'_, Roland808>) -> String {

    match event_name {
        "set_drum" => {
            state.0.lock().unwrap().drum = format!("{}", data);
        },
        _ => panic!("what event!")
    }


    format!("Hello, {}! You've been greeted from Rust!", state.0.lock().unwrap().drum.as_str())
}

fn main() {
    let shared_state = Arc::new(Mutex::new(MachineState::default()));

    let engine = Engine::new(shared_state.clone());
    engine.run();
    tauri::Builder::default()
        .manage(engine)
        .manage(Roland808(shared_state))
        .invoke_handler(tauri::generate_handler![handle_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
