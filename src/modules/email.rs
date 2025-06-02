use jh_admin_cli_macros::{Module, derive_tool};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use url::Url;
use webbrowser;

use crate::models::email::{RespData, ZohoApiResponse};

#[derive(Module)]
#[module(
    name = "Email Management Module",
    desc = "Manage Email (ZOHO) users and groups"
)]
pub struct EmailModule;

#[derive(Debug, Serialize, Deserialize)]
struct TokenInfo {
    access_token: String,
    refresh_token: String,
    last_refresh: SystemTime,
}

static mut CURRENT_ACCESS_TOKEN: Option<String> = None;
const TOKEN_FILE_PATH: &str = "zoho_refresh_token.json";
const REFRESH_INTERVAL: Duration = Duration::from_secs(30 * 60); // 30 minutes

fn get_auth_code() -> String {
    let client_id = "";
    let scope = "ZohoMail.organization.accounts.ALL";
    let redirect_uri = "http://localhost/";

    let auth_url = format!(
        "https://accounts.zoho.com/oauth/v2/auth?response_type=code&client_id={}&scope={}&redirect_uri={}&access_type=offline",
        client_id, scope, redirect_uri
    );

    println!("Opening browser for Zoho authorization...");
    webbrowser::open(&auth_url).expect("Failed to open web browser");

    println!("After authorizing, you will be redirected to a URL like: http://localhost/?code=YOUR_AUTH_CODE");
    println!("Please enter the authorization code from the URL:");

    let mut auth_code = String::new();
    std::io::stdin().read_line(&mut auth_code).expect("Failed to read line");

    // Trim whitespace and extract only the code parameter if full URL is pasted
    let auth_code = auth_code.trim();
    if auth_code.starts_with("http") {
        let url = Url::parse(auth_code).expect("Failed to parse URL");
        let code = url.query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .expect("No code found in URL");
        code
    } else {
        auth_code.to_string()
    }
}

fn exchange_auth_code_for_tokens(auth_code: &str) -> TokenInfo {
    let client_id = "";
    // let client_secret = std::env::var("ZOHO_CLIENT_SECRET").expect("ZOHO_CLIENT_SECRET not set");
    let client_secret = "";
    let redirect_uri = "http://localhost/";

    let client = Client::new();
    let response = client
        .post("https://accounts.zoho.com/oauth/v2/token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri),
            ("code", auth_code),
        ])
        .send()
        .expect("Failed to send token request");

    if !response.status().is_success() {
        panic!("Error getting tokens: {} - {}",
            response.status(),
            response.text().unwrap_or_else(|_| "No error message".to_string()));
    }

    let token_response: serde_json::Value = response.json().expect("Failed to parse token response");

    let token_info = TokenInfo {
        access_token: token_response["access_token"].as_str().expect("No access token").to_string(),
        refresh_token: token_response["refresh_token"].as_str().expect("No refresh token").to_string(),
        last_refresh: SystemTime::now(),
    };

    // Store the token info to a file
    let file_content = serde_json::to_string_pretty(&token_info).expect("Failed to serialize token info");
    let mut file = File::create(TOKEN_FILE_PATH).expect("Failed to create token file");
    file.write_all(file_content.as_bytes()).expect("Failed to write token info");

    // Store the access token for immediate use
    unsafe {
        CURRENT_ACCESS_TOKEN = Some(token_info.access_token.clone());
    }

    token_info
}

fn refresh_access_token(refresh_token: &str) -> String {
    // let client_id = std::env::var("ZOHO_CLIENT_ID").expect("ZOHO_CLIENT_ID not set");
    let client_id = "";
    // let client_secret = std::env::var("ZOHO_CLIENT_SECRET").expect("ZOHO_CLIENT_SECRET not set");
    let client_secret = "";

    let client = Client::new();
    let response = client
        .post("https://accounts.zoho.com/oauth/v2/token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .expect("Failed to send refresh token request");

    if !response.status().is_success() {
        panic!("Error refreshing token: {} - {}",
            response.status(),
            response.text().unwrap_or_else(|_| "No error message".to_string()));
    }

    let token_response: serde_json::Value = response.json().expect("Failed to parse refresh token response");
    token_response["access_token"].as_str().expect("No access token").to_string()
}

fn get_oauth_token() -> String {
    // Check if we have a valid access token in memory
    unsafe {
        // Use &raw const to create a raw pointer
        let token_ptr = &raw const CURRENT_ACCESS_TOKEN;
        if let Some(token) = (*token_ptr).as_ref() {
            return token.clone();
        }
    }

    // Check if we have a refresh token file
    if Path::new(TOKEN_FILE_PATH).exists() {
        // Read token info from file
        let mut file = File::open(TOKEN_FILE_PATH).expect("Failed to open token file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read token file");

        let mut token_info: TokenInfo = serde_json::from_str(&contents).expect("Failed to parse token info");

        // Check if we need to refresh (more than 30 minutes since last refresh)
        let now = SystemTime::now();
        if now.duration_since(token_info.last_refresh).unwrap_or(Duration::from_secs(0)) > REFRESH_INTERVAL {
            println!("Refreshing access token...");
            let new_access_token = refresh_access_token(&token_info.refresh_token);

            // Update token info
            token_info.access_token = new_access_token;
            token_info.last_refresh = now;

            // Save updated token info
            let file_content = serde_json::to_string_pretty(&token_info).expect("Failed to serialize token info");
            let mut file = File::create(TOKEN_FILE_PATH).expect("Failed to create token file");
            file.write_all(file_content.as_bytes()).expect("Failed to write token info");
        }

        // Store the access token for future use
        unsafe {
            CURRENT_ACCESS_TOKEN = Some(token_info.access_token.clone());
        }

        token_info.access_token
    } else {
        // No refresh token file, need to go through full authorization flow
        println!("No existing token found. Starting authorization process...");
        let auth_code = get_auth_code();
        let token_info = exchange_auth_code_for_tokens(&auth_code);
        token_info.access_token
    }
}

#[derive_tool(
    id = "ListEmailUsers",
    name = "ListCurrentEmailUsers",
    desc = "Lists all users in the system (ZOHO)"
)]
pub fn list_email_users() {
    let oauth_token = get_oauth_token();
    // let zoid = std::env::var("ZOHO_ZOID").expect("ZOHO_ZOID not set");
    let zoid = "";

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://mail.zoho.com/api/organization/{}/accounts?limit=100",
            zoid
        ))
        .header("Authorization", format!("Zoho-oauthtoken {}", oauth_token))
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        let users = response
            .json::<ZohoApiResponse>()
            .expect("Failed to parse response");
        match users.data.unwrap() {
            RespData::Item(e) => println!("{:?}", e.unwrap().first_name),
            RespData::Vector(v) => {
                for user in v.unwrap() {
                    println!(
                        "User: {}, Email(s): {}",
                        user.display_name.unwrap(),
                        user.email_address
                            .unwrap_or(vec![])
                            .iter()
                            .map(|e| format!(
                                "{}{}",
                                e.mail_id.clone().unwrap(),
                                if e.is_alias.unwrap() { " (Alias)" } else { "" }
                            ))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
        }
    } else {
        println!("Error: {}", response.status());
        println!("{}", response.text().expect("Failed to get response text"));
    }
}
