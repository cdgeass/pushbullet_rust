use super::Request;
use clap::Subcommand;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Globally unique identifier for this channel, chosen by the channel creator
    tag: Option<String>,

    /// Name of the channel
    name: Option<String>,

    /// Description of the channel
    description: Option<String>,

    /// Image to display for the channel
    image_url: Option<String>,

    /// Website for the channel
    website_url: Option<String>,

    /// URL for RSS feed. If this is set, the RSS feed will be used to automatically create posts for this channel
    /// #[arg(long)]
    feed_url: Option<String>,

    /// Filters to use when a feed_url is set, only posts matching these filters will be sent out on the channel.
    feed_filters: Vec<CreateFilter>,

    /// If this is set to true, a subscription will be created as soon as the channel is created.
    subscribe: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFilter {
    /// Field to match filter against, only "title" is currently supported
    field: Option<String>,

    /// Operation for filter to match value against the chosen field, only "contains" is currently supported
    operator: Option<String>,

    /// Value to compare to the field using the chosen operator
    value: Option<String>,

    /// Invert the result of this filter
    not: Option<bool>,

    /// If true, match without regards to lowercase/uppercase
    ignore_case: Option<bool>,
}

#[derive(Subcommand)]
pub enum ChannelCommands {
    /// Create a channel.
    Create {
        /// Globally unique identifier for this channel, chosen by the channel creator
        #[arg(long)]
        tag: Option<String>,

        /// Name of the channel
        #[arg(long)]
        name: Option<String>,

        /// Description of the channel
        #[arg(long)]
        description: Option<String>,

        /// Image to display for the channel
        #[arg(long)]
        image_url: Option<String>,

        /// Website for the channel
        #[arg(long)]
        website_url: Option<String>,

        /// URL for RSS feed. If this is set, the RSS feed will be used to automatically create posts for this channel
        #[arg(long)]
        feed_url: Option<String>,

        /// If this is set to true, a subscription will be created as soon as the channel is created.
        #[arg(long)]
        subscribe: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
    },
}

impl Request for ChannelCommands {
    fn build_request(
        &self,
        access_token: &str,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        match self {
            ChannelCommands::Create { tag, name, description, image_url, website_url, feed_url, subscribe, data_binary, } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => CreateRequest {
                        tag: tag.clone(),
                        name: name.clone(),
                        description: description.clone(),
                        image_url: image_url.clone(),
                        website_url: website_url.clone(),
                        feed_url: feed_url.clone(),
                        feed_filters: vec![],
                        subscribe: subscribe.clone(),
                    }
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/channels")
                    .json(&request);
                Ok(request_builder)
            }
        }
    }
}
