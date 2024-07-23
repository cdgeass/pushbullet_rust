use clap::Subcommand;
use reqwest::blocking::RequestBuilder;

use super::Request;

#[derive(Subcommand)]
pub enum UserCommands {
    /// Gets the currently logged in user.
    Get,
}

impl Request for UserCommands {
    fn build_request(
        &self,
        access_token: &str,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        match self {
            UserCommands::Get => {
                let request_bulder =
                    reqwest::blocking::Client::new().get("https://api.pushbullet.com/v2/users/me");
                Ok(request_bulder)
            }
        }
    }
}
