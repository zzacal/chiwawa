// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use reqwest::{Client, header::{HeaderMap, HeaderValue, HeaderName}};
use serde::{Deserialize, Serialize};
pub mod chi_models;
use chi_models::{ChiConfig, Action, map_to_header_map};

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
async fn send(request: Action) -> Result<SerializableResponse, SerializableError> {
    let result = send_request(request).await;
    let data: Result<SerializableResponse, SerializableError> = match result {
        Ok(_resp) => Ok(map_to_serializable_response(_resp).await),
        Err(_err) => Err(_err.into())
    };
    data
}

async fn send_request(request: Action) -> Result<reqwest::Response, reqwest::Error> {
    if request.method == "POST" {
        println!("{:?}", request);
        send_post(request).await
    } else if request.method == "PUT" {
        send_put(request).await
    } else if request.method == "PUT" {
        send_delete(request).await
    } else {
        send_get(request).await
    }
}

async fn send_get(request: Action) -> Result<reqwest::Response, reqwest::Error> {
    let body = reqwest::get(request.url).await;
    body
}


async fn send_post(request: Action) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    
    let response = client
        .post(request.url)
        .headers(map_to_header_map(request.headers))
        .body(request.body)
        .send()
        .await;
    response
}


async fn send_put(request: Action) -> Result<reqwest::Response, reqwest::Error>  {
    let client = Client::new();
    
    let response = client
        .put(request.url)
        .headers(map_to_header_map(request.headers))
        .body(request.body)
        .send()
        .await;
    response
}

async fn send_delete(request: Action) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    
    let response = client
        .delete(request.url)
        .headers(map_to_header_map(request.headers))
        .body(request.body)
        .send()
        .await;
    response
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize, greet, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use reqwest::Response;
#[derive(Serialize, Deserialize, Debug)]
struct SerializableResponse {
    // Add fields from the Response struct that you want to serialize
    // For example, you can include fields like status, headers, etc.
    // For simplicity, let's assume we are only serializing the status code.
    status: u16,
    headers: Vec<(String, String)>,
    body: String
}

impl From<Response> for SerializableResponse {
    fn from(response: Response) -> Self {
        // Extract the necessary information from the response and populate the fields
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| (String::from(name.to_string()), String::from(value.to_str().unwrap_or(""))))
            .collect();

        SerializableResponse { 
            status,
            headers,
            body: "".to_string()
        }
    }
}

async fn map_to_serializable_response(response: Response) -> SerializableResponse {
        // Extract the necessary information from the response and populate the fields
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| (String::from(name.to_string()), String::from(value.to_str().unwrap_or(""))))
            .collect();
        let body = response.text().await.expect("");

        SerializableResponse { 
            status,
            headers,
            body
        }
}

use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
struct SerializableError {
    // Add fields from the Error struct that you want to serialize
    // For example, you can include fields like source, url, etc.
    // For simplicity, let's assume we are only serializing the error message.
    message: String,
}

impl From<Error> for SerializableError {
    fn from(error: Error) -> Self {
        // Extract the necessary information from the error and populate the fields
        let message = error.to_string();
        
        SerializableError { message }
    }
}
