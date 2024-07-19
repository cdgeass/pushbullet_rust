use std::error::Error;

use clap::Subcommand;
use serde::Serialize;
use serde_json::Value;

use super::{PaginationArgs, Request};

#[derive(Subcommand)]
pub enum DeviceCommands {
    /// Get a list of devices belonging to the current user.
    List(PaginationArgs),

    /// Create a new device.
    Create {
        /// Name to use when displaying the device
        #[arg(long)]
        nickname: Option<String>,

        /// Model of the device
        #[arg(long)]
        model: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        manufacturer: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        push_token: Option<String>,

        /// Version of the Pushbullet application installed on the device
        #[arg(long)]
        app_version: Option<i32>,

        /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"S
        #[arg(long)]
        icon: Option<String>,

        /// true if the devices has SMS capability, currently only true for type="android" devices
        #[arg(long)]
        has_sms: Option<String>,

        /// reques with body like {"app_version":8623,"manufacturer":"Apple","model":"iPhone 5s (GSM)","nickname":"Elon Musk's iPhone","push_token":"production:f73be0ee7877c8c7fa69b1468cde764f"}'
        #[arg(long)]
        body: Option<String>,
    },

    /// Update an existing device.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// Name to use when displaying the device
        #[arg(long)]
        nickname: Option<String>,

        /// Model of the device
        #[arg(long)]
        model: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        manufacturer: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        push_token: Option<String>,

        /// Version of the Pushbullet application installed on the device
        #[arg(long)]
        app_version: Option<i32>,

        /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"S
        #[arg(long)]
        icon: Option<String>,

        /// true if the devices has SMS capability, currently only true for type="android" devices
        #[arg(long)]
        has_sms: Option<String>,
    },

    /// Delete a device.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

#[derive(Debug, Serialize)]
struct Device {
    nickname: Option<String>,
    model: Option<String>,
    manufacturer: Option<String>,
    push_token: Option<String>,
    app_version: Option<i32>,
    icon: Option<String>,
    has_sms: Option<String>,
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
                    }
                    None => {
                        let device = Device {
                            nickname: nickname.clone(),
                            model: model.clone(),
                            manufacturer: manufacturer.clone(),
                            push_token: push_token.clone(),
                            app_version: app_version.clone(),
                            icon: icon.clone(),
                            has_sms: has_sms.clone(),
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
                    nickname: nickname.clone(),
                    model: model.clone(),
                    manufacturer: manufacturer.clone(),
                    push_token: push_token.clone(),
                    app_version: app_version.clone(),
                    icon: icon.clone(),
                    has_sms: has_sms.clone(),
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