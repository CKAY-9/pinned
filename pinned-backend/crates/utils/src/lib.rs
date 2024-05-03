use actix_web::HttpRequest;
use chrono::prelude::{ DateTime, Utc };
use std::env;

pub fn get_env_var(key: &str) -> String {
    let env_var: String = env::var(key).unwrap_or_else(|e| {
        println!("{}", e);
        "".to_owned()
    });
    env_var
}

pub fn get_discord_api_url() -> String {
    "https://discord.com/api/v10".to_string()
}

pub fn get_local_api_url() -> String {
    let api: String = get_env_var("LOCAL_HOST") + "/api/v1";
    api
}

pub fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

pub fn extract_header_value(request: &HttpRequest, header_key: &str) -> Option<String> {
    let auth_header = request.headers().get(header_key);
    if auth_header.is_none() {
        return None;
    }
    Some(auth_header.unwrap().to_str().unwrap().to_string())
}