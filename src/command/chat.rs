use std::error::Error;

use clap::Subcommand;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::{PaginationArgs, Request};

#[derive(Subcommand)]
pub enum ChatCommands {
    /// Get a list of chats belonging to the current user.
    List(PaginationArgs),

    /// Create a chat with another user or email address if one does not already exist.
    Create {
        /// Email of person to create chat with (does not have to be a Pushbullet user)
        #[arg(long)]
        email: Option<String>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Update existing chat object.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(long)]
        muted: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Delete a chat object.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Email of person to create chat with (does not have to be a Pushbullet user)
    /// Example: "carmack@idsoftware.com"
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// true to mute the grant, false to unmute it
    muted: Option<bool>,
}

impl Request for ChatCommands {
    fn build_request(&self, access_token: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        match self {
            ChatCommands::List(args) => {
                let request_builder = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/chats")
                    .query(&args.to_query());
                Ok(request_builder)
            }
            ChatCommands::Create { email, data_binary } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => CreateRequest {
                        email: email.clone(),
                    },
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/chats")
                    .json(&request);
                Ok(request_builder)
            }
            ChatCommands::Update {
                iden,
                muted,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => UpdateRequest {
                        muted: muted.clone(),
                    },
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post(format!("https://api.pushbullet.com/v2/chats/{}", iden))
                    .json(&request);
                Ok(request_builder)
            }
            ChatCommands::Delete { iden } => {
                let request_builder = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/chats/{}", iden));
                Ok(request_builder)
            }
        }
    }
}
