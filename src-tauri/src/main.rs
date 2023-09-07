// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use reqwest::{Client, header::{HeaderMap, HeaderValue, HeaderName}};
use serde::{Deserialize, Serialize};

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
async fn send(request: Action) -> String {
    if request.method == "POST" {
        println!("{:?}", request);
        send_post(request).await.expect("some msg")
    } else if request.method == "PUT" {
        send_put(request).await
    } else {
        send_get(request).await
    }
}

async fn send_get(request: Action) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}


async fn send_post(request: Action) -> Result<String, reqwest::Error> {
    let client = Client::new();
    
    let response = client
        .post(request.url)
        .headers(map_to_header_map(request.headers))
        .body(request.body)
        .send()
        .await?;
    
    let response_text = response.text().await?;
    
    Ok(response_text)
}


async fn send_put(request: Action) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

async fn send_delete(request: Action) -> String {
    let body = reqwest::get(request.url).await.expect("").text().await.expect("");
    body
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize, greet, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChiRequest {
    method: String,
    url: String,
}

#[derive(Serialize)]
pub struct ChiConfig {
    methods: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnabledKvp<TKey, TVal> {
    is_enabled: bool,
    key: TKey,
    value: TVal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    method: String,
    url: String,
    name: String,
    headers: Option<Vec<EnabledKvp<String, String>>>,
    parameters: Option<Vec<EnabledKvp<String, String>>>,
    body: String
}


fn map_to_header_map(headers: Option<Vec<EnabledKvp<String, String>>>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    
    if let Some(enabled_kvps) = headers {
        for kvp in enabled_kvps {
            if kvp.is_enabled {
                if let Ok(header_value) = HeaderValue::from_str(&kvp.value) {
                    if let Ok(header_name) = HeaderName::from_str(&kvp.key) {
                        header_map.insert(header_name, header_value);
                    }
                }
            }
        }
    }
    
    header_map
}
