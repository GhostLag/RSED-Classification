// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs;
use reqwest::header::USER_AGENT;
use reqwest;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("{}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn classification(address: &str) -> String {
    format!("{}", address)
}

async fn sa1_request(client: reqwest::Client) -> String {
    let url = "https://www.abs.gov.au/census/find-census-data/search-by-area/addresscoder";
    let params = HashMap::from([
        ("addressline1", "83 Prince Street"),
        ("locality", "Canley Heights"),
        ("state", "NSW"),
        ("postcode", "2166"),
        ("cycle", "2021"),
    ]);
    let url = reqwest::Url::parse_with_params(url, &params).unwrap();

    let res: String = client.get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; rv:110.0) Gecko/20100101 Firefox/110.0")
        .send()
        .await
        .unwrap() // if this responds with an error this entire thing will die
        .text()
        .await
        .unwrap_or("Error: address not found".to_string());
    
    let sa1: serde_json::Value = serde_json::from_str(&res).unwrap();

    sa1.get("geographyList").unwrap()[17]["code"].as_str().unwrap_or("Error").to_string()
}

fn sa1_lookup(sa1: u64) -> Option<u8> {
    let sa1_json: serde_json::Value = serde_json::from_str(&fs::read_to_string("../sa1.json").expect("File should exist?")).unwrap();
    match sa1_json.get(format!("{}", sa1)) {
        Some(x) => Some((x.as_number().unwrap().as_u64().unwrap() as u8).into()),
        None => None
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res: u64 = u64::from_str_radix(&sa1_request(client).await, 10).unwrap();
    println!("{}", sa1_lookup(res).unwrap());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, classification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
