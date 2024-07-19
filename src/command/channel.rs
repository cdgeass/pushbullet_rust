use clap::{Args, Subcommand};
use serde::Serialize;
use super::Request;

#[derive(Debug, Serialize, Args)]
pub struct CreateRequest {
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
    /// #[arg(long)]
    /// feed_url: Option<String>,

    /// Filters to use when a feed_url is set, only posts matching these filters will be sent out on the channel.
    /// feed_filters: Vec<CreateFilter>,
    
    /// If this is set to true, a subscription will be created as soon as the channel is created.
    #[arg(long)]
    subscribe: Option<bool>,
}

#[derive(Debug, Serialize)]
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
    Create(CreateRequest),
}

impl Request for ChannelCommands {

    fn request(&self, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            ChannelCommands::Create(request) => {
                let response_result = reqwest::blocking::Client::new()
                .post("https://api.pushbullet.com/v2/channels")
                .header("Access-Token", access_token)
                .json(&request)
                .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            },
        }
    }
}