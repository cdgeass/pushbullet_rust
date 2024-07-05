use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::Path,
};

use serde::Serialize;
use serde_json::Value;

use super::{ChatCommands, DeviceCommands};

pub fn set_access_token(access_token: &str) -> io::Result<()> {
    let home = env::var("HOME").unwrap();
    let path = Path::new(&home).join(".config").join("pbr").join("config");

    let mut config_file = match File::open(&path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            match fs::create_dir_all(&path.parent().unwrap()) {
                Ok(()) => match File::create(&path) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    };

    config_file.write_all(access_token.as_bytes())?;

    Ok(())
}

pub fn read_access_token() -> io::Result<String> {
    let home = env::var("HOME").unwrap();
    let path = Path::new(&home).join(".config").join("pbr").join("config");
    Ok(fs::read_to_string(path)?)
}

pub trait Request {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>>;
}

impl Request for ChatCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
        match self {
            ChatCommands::List(args) => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .get("https://api.pushbullet.com/v2/chats")
                    .header("Access-Token", access_token);

                let mut query: Vec<(String, String)> = vec![];
                if let Some(cursor) = &args.cursor {
                    query.push((String::from("cursor"), cursor.to_string()));
                }
                if let Some(limit) = &args.limit {
                    query.push((String::from("limit"), limit.to_string()));
                }
                request_builder = request_builder.query(&query);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            ChatCommands::Create { email } => {
                let mut map = HashMap::new();
                map.insert("email", email);

                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .post("https://api.pushbullet.com/v2/chats")
                    .header("Access-Token", access_token)
                    .json(&map);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            ChatCommands::Update { iden, muted } => {
                let mut map = HashMap::new();
                if let Some(muted) = muted {
                    map.insert("muted", muted);
                }

                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .post(format!("https://api.pushbullet.com/v2/chats/{}", iden))
                    .header("Access-Token", access_token)
                    .json(&map);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            ChatCommands::Delete { iden } => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete(format!("https://api.pushbullet.com/v2/chats/{}", iden))
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
        }
    }
}

#[derive(Serialize)]
struct Device<'a> {
    nickname: &'a Option<String>,
    model: &'a Option<String>,
    manufacturer: &'a Option<String>,
    push_token: &'a Option<String>,
    app_version: &'a Option<i32>,
    icon: &'a Option<String>,
    has_sms: &'a Option<String>,
}

impl Request for DeviceCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
        match self {
            DeviceCommands::List(args) => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .get("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                let mut query: Vec<(String, String)> = vec![];
                if let Some(cursor) = &args.cursor {
                    query.push((String::from("cursor"), cursor.to_string()));
                }
                if let Some(limit) = &args.limit {
                    query.push((String::from("limit"), limit.to_string()));
                }
                request_builder = request_builder.query(&query);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            DeviceCommands::Create {
                nickname,
                model,
                manufacturer,
                push_token,
                app_version,
                icon,
                has_sms,
                body,
            } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                if let Some(body) = body {
                    let json: Value = match serde_json::from_str(body) {
                        Ok(json) => json,
                        Err(e) => return Err(Box::new(e)),
                    };
                    request_builder = request_builder.json(&json);
                } else {
                    let device = Device {
                        nickname,
                        model,
                        manufacturer,
                        push_token,
                        app_version,
                        icon,
                        has_sms,
                    };
                    request_builder = request_builder.json(&device);
                }

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            DeviceCommands::Update {
                iden,
                nickname,
                model,
                manufacturer,
                push_token,
                app_version,
                icon,
                has_sms,
            } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post(format!("https://api.pushbullet.com/v2/devices/{}", iden))
                    .header("Access-Token", access_token);

                let device = Device {
                    nickname,
                    model,
                    manufacturer,
                    push_token,
                    app_version,
                    icon,
                    has_sms,
                };
                request_builder = request_builder.json(&device);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            DeviceCommands::Delete { iden } => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete(format!("https://api.pushbullet.com/v2/devices/{}", iden))
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
        }
    }
}
