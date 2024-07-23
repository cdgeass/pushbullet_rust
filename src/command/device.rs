use std::error::Error;

use clap::Subcommand;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

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
        has_sms: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
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
        has_sms: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>
    },

    /// Delete a device.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Name to use when displaying the device
    /// Example: "Elon Musk's iPhone"
    nickname: Option<String>,

    /// Model of the device
    /// Example: "iPhone 5s (GSM)"
    model: Option<String>,

    /// Manufacturer of the device
    /// Example: "Apple"
    manufacturer: Option<String>,

    /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
    /// Example: "production:f73be0ee7877c8c7fa69b1468cde764f"
    push_token: Option<String>,

    /// Version of the Pushbullet application installed on the device
    /// Example: 8623
    app_version: Option<i32>,

    /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"
    /// Example: "ios"
    icon: Option<String>,

    /// true if the devices has SMS capability, currently only true for type="android" devices
    /// Example: true
    has_sms: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// Name to use when displaying the device
    /// Example: "Elon Musk's iPhone"
    nickname: Option<String>,

    /// Model of the device
    /// Example: "iPhone 5s (GSM)"
    model: Option<String>,

    /// Manufacturer of the device
    /// Example: "Apple"
    manufacturer: Option<String>,

    /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
    /// Example: "production:f73be0ee7877c8c7fa69b1468cde764f"
    push_token: Option<String>,

    /// Version of the Pushbullet application installed on the device
    /// Example: 8623
    app_version: Option<i32>,

    /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"
    /// Example: "ios"
    icon: Option<String>,

    /// true if the devices has SMS capability, currently only true for type="android" devices
    /// Example: true
    has_sms: Option<bool>,
}

impl Request for DeviceCommands {
    fn build_request(&self, access_token: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        match self {
            DeviceCommands::List(args) => {
                let request_builder = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/devices")
                    .query(&args.to_query());
                Ok(request_builder)
            }
            DeviceCommands::Create {
                nickname,
                model,
                manufacturer,
                push_token,
                app_version,
                icon,
                has_sms,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        },
                    },
                    None => CreateRequest {
                        nickname: nickname.clone(),
                        model: model.clone(),
                        manufacturer: manufacturer.clone(),
                        push_token: push_token.clone(),
                        app_version: app_version.clone(),
                        icon: icon.clone(),
                        has_sms: has_sms.clone(),
                    }
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/devices")
                    .json(&request);
                Ok(request_builder)
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
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        },
                    },
                    None => UpdateRequest {
                        nickname: nickname.clone(),
                        model: model.clone(),
                        manufacturer: manufacturer.clone(),
                        push_token: push_token.clone(),
                        app_version: app_version.clone(),
                        icon: icon.clone(),
                        has_sms: has_sms.clone(),
                    }
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post(format!("https://api.pushbullet.com/v2/devices/{}", iden))
                    .json(&request);
                Ok(request_builder)
            }
            DeviceCommands::Delete { iden } => {
                let request_builder = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/devices/{}", iden));
                Ok(request_builder)
            }
        }
    }
}