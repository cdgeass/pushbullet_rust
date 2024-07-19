use std::collections::HashMap;

use clap::Subcommand;

use super::{PaginationArgs, Request};

#[derive(Subcommand)]
pub enum SubscriptionCommands {
    /// Get a list of subscriptions belonging to the current user.
    List(PaginationArgs),

    Create {
        /// Unique tag for the channel to subscribe to
        #[arg(long)]
        channel_tag: Option<String>,
    },

    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(long)]
        muted: Option<bool>,
    },

    Delete {
        /// Unique identifier for this object
        iden: String,
    },

    ChannelInfo {
        /// Tag of the channel to get information for
        #[arg(long)]
        tag: Option<String>,

        /// Don't show recent pushes, defaults to false
        #[arg(long)]
        no_recent_pushed: Option<bool>,
    },
}

impl Request for SubscriptionCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            SubscriptionCommands::List(request) => {
                let mut query = vec![];
                if let Some(cursor) = &request.cursor {
                    query.push((String::from("cursor"), cursor.to_owned()));
                }
                if let Some(limit) = request.limit {
                    query.push((String::from("limit"), limit.to_string()));
                }

                let response_result = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/subscriptions")
                    .header("Access-Token", access_token)
                    .query(&query)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            SubscriptionCommands::Create { channel_tag } => {
                let mut map = HashMap::new();
                if let Some(channel_tag) = channel_tag {
                    map.insert(String::from("channel_tag"), channel_tag.to_owned());
                }

                let response_result = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/subscriptions")
                    .header("Access-Token", access_token)
                    .json(&map)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            SubscriptionCommands::Update { iden, muted } => {
                let mut map = HashMap::new();
                if let Some(muted) = muted {
                    map.insert(String::from("muted"), muted.to_owned());
                }

                let response_result = reqwest::blocking::Client::new()
                    .post(format!(
                        "https://api.pushbullet.com/v2/subscriptions/{}",
                        iden
                    ))
                    .header("Access-Token", access_token)
                    .json(&map)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            SubscriptionCommands::Delete {iden} => {
                let response_result = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/subscriptions/{}", iden))
                    .header("Access-Token", access_token)
                    .send();

                match response_result {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            },
            SubscriptionCommands::ChannelInfo { tag, no_recent_pushed, } => {
                let mut query = vec![];
                if let Some(tag) = tag {
                    query.push(tag.to_owned());
                }
                if let Some(no_recent_pushed) = no_recent_pushed {
                    query.push(no_recent_pushed.to_string());
                }

                let response_result = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/channel-info")
                    .header("Access-Token", access_token)
                    .send();

                match response_result {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            },
        }
    }
}
