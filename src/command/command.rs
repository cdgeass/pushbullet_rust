use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
};

use clap::{Args, Parser, Subcommand};
use reqwest::blocking::{multipart, Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    channel::ChannelCommands, chat::ChatCommands, device::DeviceCommands, push::PushCommands,
    subscription::SubscriptionCommands, text::TextCommands, user::UserCommands,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Serialize)]
pub struct PaginationArgs {
    /// When listing objects, if you receive a cursor in the response, it means the results are on multiple pages. To request the next page of results, use this cursor as the parameter cursor in the next request.
    #[arg(long)]
    pub cursor: Option<String>,

    /// You can specify a limit parameter that return a list of objects to get a smaller number of objects on each page.
    #[arg(long, default_value = "500")]
    pub limit: Option<i32>,
}

impl PaginationArgs {
    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query: Vec<(String, String)> = vec![];
        if let Some(cursor) = &self.cursor {
            query.push((String::from("cursor"), cursor.to_owned()));
        }
        if let Some(limit) = self.limit {
            query.push((String::from("limit"), limit.to_string()));
        }
        query
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// To access the API you'll need an access token so the server knows who you are.
    AccessToken {
        /// You can get one from your Account Settings page.
        access_token: String,
    },

    /// Chats are created whenever you send a message to someone or a receive a message from them and there is no existing chat between you and the other user.
    #[command(subcommand)]
    Chat(ChatCommands),

    #[command(subcommand)]
    Device(DeviceCommands),

    /// A Push.
    #[command(subcommand)]
    Push(PushCommands),

    #[command(subcommand)]
    Channel(ChannelCommands),

    /// Subscribe to channels to receive any updates pushed to that channel.
    #[command(subcommand)]
    Subscription(SubscriptionCommands),

    /// Send text messages (SMS) to one phone number or group messages (MMS) to mulitple phone numbers. Also supports sending picture messages. Text messages are queued and sent as soon as possible. If the sending device does not come online and sync within 1 hour, the message is canceled and will not send.
    #[command(subcommand)]
    Text(TextCommands),

    #[command(subcommand)]
    User(UserCommands),
}

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

#[derive(Debug, Deserialize)]
pub struct UploadRequestResponse {
    pub file_name: String,
    pub file_type: String,
    pub file_url: String,
    pub upload_url: String,
}

pub fn upload_request(
    access_token: &str,
    file_name: String,
    file_type: Option<String>,
) -> Result<UploadRequestResponse, Box<dyn Error>> {
    let path_buf = PathBuf::from(&file_name);

    let final_file_type: String;
    match file_type {
        Some(file_type) => final_file_type = file_type,
        None => {
            let flags = magic::cookie::Flags::MIME_TYPE;
            let cookie = magic::Cookie::open(flags).unwrap();

            let database = &Default::default();
            let cookie = cookie.load(database).unwrap();
            final_file_type = cookie.file(&file_name).unwrap();
        }
    }

    let request_builder = reqwest::blocking::Client::new()
        .post("https://api.pushbullet.com/v2/upload-request")
        .header("Access-Token", access_token)
        .json(&json!({
            "file_name": path_buf.file_name(),
            "file_type": final_file_type,
        }));

    match request_builder.send() {
        Ok(res) => match res.text() {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(value) => Ok(value),
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn upload(access_token: &str, file_name: &str, upload_url: &str) -> Result<(), Box<dyn Error>> {
    let form = match multipart::Form::new().file("file", &file_name) {
        Ok(form) => form,
        Err(error) => {
            return Err(Box::new(error));
        }
    };
    let response_result = reqwest::blocking::Client::new()
        .post(upload_url)
        .header("Access-Token", access_token)
        .multipart(form)
        .send();
    match response_result {
        Ok(_) => Ok(()),
        Err(error) => {
            return Err(Box::new(error));
        }
    }
}

pub trait Request {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
        let request_builder = match self.build_request(access_token) {
            Ok(request_builder) => request_builder.header("Access-Token", access_token),
            Err(error) => {
                return Err(error);
            }
        };

        match request_builder.send() {
            Ok(response) => match response.text() {
                Ok(text) => Ok(text),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => Err(Box::new(error)),
        }
    }

    fn build_request(&self, access_token: &str) -> Result<RequestBuilder, Box<dyn Error>>;
}
