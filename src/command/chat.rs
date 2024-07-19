use std::{collections::HashMap, error::Error};

use clap::Subcommand;

use super::{PaginationArgs, Request};

#[derive(Subcommand)]
pub enum ChatCommands {
    /// Get a list of chats belonging to the current user.
    List(PaginationArgs),

    /// Create a chat with another user or email address if one does not already exist.
    Create {
        /// Email of person to create chat with (does not have to be a Pushbullet user)
        #[arg(long)]
        email: String,
    },

    /// Update existing chat object.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(long)]
        muted: Option<bool>,
    },

    /// Delete a chat object.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
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
                    query.push((String::from("cursor"), cursor.to_owned()));
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
                map.insert("email", email.to_owned());

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
                    map.insert("muted", muted.to_owned());
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
