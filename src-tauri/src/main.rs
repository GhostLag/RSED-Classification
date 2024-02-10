// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::header::USER_AGENT;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Form {
    address: String,
    locality: String,
    state: String,
    postcode: String,
    cycle: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn classification(address: &str, cycle: bool) -> String {
    let address_json: serde_json::Result<serde_json::Value> = serde_json::from_str(address);
    match address_json {
        Ok(x) => {
            let formatted = |x: serde_json::Value| {
                let form = build_form(&x, &cycle);

                match sa1_request(form).parse::<u64>() {
                    Ok(n) => match sa1_lookup(n, &cycle) {
                        Some(u) => match u <= 25 {
                            true => format!(
                                "You are eligible as your area is in the {} percentile",
                                ordinal(u)
                            ),
                            false => format!(
                                "You are ineligible as your area is in the {} percentile",
                                ordinal(u)
                            ),
                        },
                        None => "Street number needed".to_string(),
                    },
                    Err(_) => "Unkown error".to_string(),
                }
            };
            formatted(x)
        }
        Err(_) => "Enter a valid address".to_string(),
    }
}

fn sa1_request(form: Form) -> String {
    let client = reqwest::blocking::Client::new();

    let url = "https://www.abs.gov.au/census/find-census-data/search-by-area/addresscoder";
    let params = HashMap::from([
        ("addressline1", form.address),
        ("locality", form.locality),
        ("state", form.state),
        ("postcode", form.postcode),
        ("cycle", form.cycle),
    ]);
    let url = reqwest::Url::parse_with_params(url, &params).unwrap();

    let res: String = client
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; rv:110.0) Gecko/20100101 Firefox/110.0",
        )
        .send()
        .unwrap() // if this responds with an error this entire thing will die
        .text()
        .unwrap_or("Error: address not found".to_string());

    let sa1: serde_json::Value = serde_json::from_str(&res).unwrap();

    match sa1.get("geographyList").unwrap().as_array() {
        Some(x) => sa1.get("geographyList").unwrap()[x.len() - 1]["code"]
            .as_str()
            .unwrap()
            .to_string(),
        None => "1".to_string(),
    }
}

fn sa1_lookup(n: u64, cycle: &bool) -> Option<u8> {
    let json = include_str!("../../sa1.json");
    let sa1_json: serde_json::Value =
        serde_json::from_str(json)
            .unwrap();

    let sa1_json = match cycle {
        true => sa1_json.get("2021").unwrap(),
        false => sa1_json.get("2016").unwrap(),
    };

    sa1_json
        .get(format!("{}", n))
        .map(|x| x.as_number().unwrap().as_u64().unwrap() as u8)
}

fn build_form(form: &serde_json::Value, year: &bool) -> Form {
    let len = form.as_array().unwrap().len();
    let address = match form[0]["long_name"]
        .to_string()
        .replace('"', "")
        .parse::<u16>()
    {
        Ok(n) => format!("{} {}", n, form[1]["long_name"]),
        Err(_) => format!("{}", form.clone()[0]["long_name"]),
    }
    .replace('"', "");
    let locality = match address.split_whitespace().count() == 1 {
        true => form[1]["long_name"].to_string().replace('"', ""),
        false => form[2]["long_name"].to_string().replace('"', ""),
    };
    let state = form[len - 3]["short_name"].to_string().replace('"', "");
    let postcode = form[len - 1]["long_name"].to_string().replace('"', "");

    Form {
        address,
        locality,
        state,
        postcode,
        cycle: match year {
            false => "2016".to_string(),
            true => "2021".to_string(),
        },
    }
}

fn ordinal(n: u8) -> String {
    match n % 10 {
        1 => format!("{}st", n),
        2 => format!("{}nd", n),
        3 => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![classification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
