#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod sound;
mod engine;

use std::sync::{Mutex, Arc};
use tauri::{State, Manager};
use engine::Engine;
use crate::engine::MachineState;
use tracing::{info};

#[derive(Default)]
pub struct Roland808(Arc<Mutex<MachineState>>);

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_event(event_name: &str, data: &str, state: State<'_, Roland808>) -> String {

    println!("received event: {} with data: {}", event_name, data);

    match event_name {
        "variation-changed" => {
            state.0.lock().unwrap().current_variation = data.to_string();
        },
        _ => panic!("what event!")
    }


    format!("Hello! You've been greeted from Rust!")
}

fn main() {
    let shared_state = Arc::new(Mutex::new(MachineState::default()));
    let (async_proc_output_tx, async_proc_output_rx) = std::sync::mpsc::channel();

    let engine = Engine::new(shared_state.clone(), async_proc_output_tx);
    engine.run();
    
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            std::thread::spawn(move || {
                loop {
                    if let Ok(output) = async_proc_output_rx.recv() {
                        rs2js(output, &app_handle);
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
fn rs2js<R: tauri::Runtime>(message: usize, manager: &impl Manager<R>) {
    tracing::info!(?message, "rs2js");
    manager
        .emit_all("rs2js", message)
        .unwrap();
}

// The Tauri command that gets called when Tauri `invoke` JavaScript API is
// called
// #[tauri::command]
// async fn js2rs(
//     message: String,
//     state: tauri::State<'_, AsyncProcInputTx>,
// ) -> Result<(), String> { 
//     info!(?message, "js2rs");
//     Ok(())
// }

