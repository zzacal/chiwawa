// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn initialize() -> ChiConfig {
    ChiConfig { methods: vec![String::from("GET"), String::from("POST"), String::from("PUT"), String::from("DELETE")] }
}

#[tauri::command]
async fn send(request: ChiRequest) -> String {
    if request.method == "POST" {
        send_post(request).await
    } else if request.method == "PUT" {
        send_put(request).await
    } else {
        send_get(request).await
    }
}

async fn send_get(request: ChiRequest) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

async fn send_post(request: ChiRequest) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

async fn send_put(request: ChiRequest) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

async fn send_delete(request: ChiRequest) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize, greet, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ChiRequest {
    method: String,
    url: String,
}

#[derive(Serialize)]
pub struct ChiConfig {
    methods: Vec<String>
}
