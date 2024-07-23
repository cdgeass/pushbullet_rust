use clap::Subcommand;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::{PaginationArgs, Request};

#[derive(Subcommand)]
pub enum SubscriptionCommands {
    /// Get a list of subscriptions belonging to the current user.
    List(PaginationArgs),

    Create {
        /// Unique tag for the channel to subscribe to
        #[arg(long)]
        channel_tag: Option<String>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(long)]
        muted: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Unique tag for the channel to subscribe to
    channel_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// true to mute the grant, false to unmute it
    muted: Option<bool>,
}

impl Request for SubscriptionCommands {
    fn build_request(
        &self,
        access_token: &str,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        match self {
            SubscriptionCommands::List(args) => {
                let request_builder = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/subscriptions")
                    .query(&args.to_query());
                Ok(request_builder)
            }
            SubscriptionCommands::Create {
                channel_tag,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => CreateRequest {
                        channel_tag: channel_tag.clone(),
                    },
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/subscriptions")
                    .json(&request);
                Ok(request_builder)
            }
            SubscriptionCommands::Update {
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
                    .post(format!(
                        "https://api.pushbullet.com/v2/subscriptions/{}",
                        iden
                    ))
                    .json(&request);
                Ok(request_builder)
            }
            SubscriptionCommands::Delete { iden } => {
                let request_builder = reqwest::blocking::Client::new().delete(format!(
                    "https://api.pushbullet.com/v2/subscriptions/{}",
                    iden
                ));
                Ok(request_builder)
            }
            SubscriptionCommands::ChannelInfo {
                tag,
                no_recent_pushed,
            } => {
                let mut query = vec![];
                if let Some(tag) = tag {
                    query.push(tag.to_owned());
                }
                if let Some(no_recent_pushed) = no_recent_pushed {
                    query.push(no_recent_pushed.to_string());
                }
                let request_builder = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/channel-info")
                    .query(&query);
                Ok(request_builder)
            }
        }
    }
}
