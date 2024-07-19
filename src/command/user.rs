use clap::Subcommand;

use super::Request;

#[derive(Subcommand)]
pub enum UserCommands {
    /// Gets the currently logged in user.
    Get,
}

impl Request for UserCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            UserCommands::Get => {
                let response_result = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/users/me")
                    .header("Access-Token", access_token)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
        }
    }
}
