// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod chi_models;
use chi_models::{ChiConfig, Action, SerializableError, SerializableResponse, map_to_serializable_response};

pub mod http_client;
use http_client::send_request;

pub mod store;
use store::get_config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn initialize() -> ChiConfig {
    ChiConfig { 
        methods: vec![String::from("GET"), String::from("POST"), String::from("PUT"), String::from("DELETE")],
        libraries: get_config()
    }
}

#[tauri::command]
async fn send(request: Action) -> Result<SerializableResponse, SerializableError> {
    let result = send_request(request).await;
    let data: Result<SerializableResponse, SerializableError> = match result {
        Ok(_resp) => Ok(map_to_serializable_response(_resp).await),
        Err(_err) => Err(_err.into())
    };
    data
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize, greet, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
