use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::Path,
};

use serde::Serialize;
use serde_json::{json, Value};

use super::{ChatCommands, DeviceCommands, PushCommands};

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

                let json_data = match body {
                    Some(body) => {
                        let json: Value = serde_json::from_str(body)?;
                        json
                    },
                    None => {
                        let device = Device {
                        nickname,
                        model,
                        manufacturer,
                        push_token,
                        app_version,
                        icon,
                        has_sms,
                        };
                        serde_json::to_value(&device)?
                    }
                };
                request_builder = request_builder.json(&json_data);

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

impl Request for PushCommands {

    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
        match self {
            PushCommands::List(args) => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .get("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                let mut query: Vec<(String, String)> = vec![];
                if let Some(modified_after) = &args.modified_after {
                    query.push((String::from("modified_after"), modified_after.to_string()));
                }
                if let Some(active) = &args.active {
                    query.push((String::from("active"), active.to_string()));
                }
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
            PushCommands::Create {
                t,
                title,
                body,
                url,
                file_name,
                file_type,
                file_url,
                source_device_iden,
                device_iden,
                client_iden,
                channel_tag,
                email,
                guid,
            } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                let mut map = HashMap::new();
                map.insert("type", t);
                if let Some(source_device_iden) = source_device_iden {
                    map.insert("source_device_iden", source_device_iden);
                }
                if let Some(device_iden) = device_iden {
                    map.insert("device_iden", device_iden);
                }
                if let Some(client_iden) = client_iden {
                    map.insert("client_iden", client_iden);
                }
                if let Some(channel_tag) = channel_tag {
                    map.insert("channel_tag", channel_tag);
                }
                if let Some(email) = email  {
                    map.insert("email", email);
                }
                if let Some(guid) = guid {
                    map.insert("guid", guid);
                }

                match t.as_str() {
                    "note" => {
                        if let Some(title) = title {
                            map.insert("title", title);
                        }
                        if let Some(body) = body {
                            map.insert("body", body);
                        }
                    },
                    "link" => {
                        if let Some(title) = title {
                            map.insert("title", title);
                        }
                        if let Some(body) = body {
                            map.insert("body", body);
                        }
                        if let Some(url) = url {
                            map.insert("url", url);
                        }
                    },
                    "file" => {
                        todo!()
                    },
                    _ => (),
                };
                request_builder = request_builder.json(&map);
                
                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::Update {
                iden,
                dismissed,
            } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post(format!("https://api.pushbullet.com/v2/devices/{}", iden))
                    .header("Access-Token", access_token);

                match dismissed {
                    Some(dismissed) => {
                        request_builder = request_builder.json(&json!({"dismissed": dismissed}));
                    },
                    _ => (),
                }

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::Delete { iden  } => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete(format!("https://api.pushbullet.com/v2/pushes/{}", iden))
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::DeleteAll => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete("https://api.pushbullet.com/v2/pushes")
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
        }
    }
}
