use clap::Subcommand;
use serde::{Deserialize, Serialize};

use super::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    /// The device_iden of the Device to send the message. This device must have SMS Android permissions granted.
    target_device_iden: Option<String>,

    /// A list of 1 more phone numbers to send this message to. Including than one number sends a group MMS message.
    address: Option<String>,

    /// The text content of the text message.
    message: Option<String>,

    /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
    guid: Option<String>,

    /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
    status: Option<String>,

    /// The mime type of the file_url being sent with this message. Only required for messages sending a file.
    file_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Map of values specifying this text message.
    data: Option<Data>,

    /// File download url for an image to send with the text message.
    file_url: Option<String>,

    /// If set to false, delete the attached file when the Text is deleted.
    skip_delete_file: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// Map of values specifying this text message.
    data: Option<Data>,

    /// When the text is deleted, don't delete the attached file. The file being deleted or not does not affect the MMS that was sent. Can only be set to true
    skip_delete_file: Option<bool>,
}

#[derive(Subcommand)]
pub enum TextCommands {
    /// Create a new text. The text will automatically be deleted after an hour whether it has been sent or not.
    Create {
        /// Map of values specifying this text message.
        #[arg(long)]
        data: Option<String>,

        /// File download url for an image to send with the text message.
        #[arg(long)]
        file_url: Option<String>,

        /// If set to false, delete the attached file when the Text is deleted.
        #[arg(long)]
        skip_delete_file: Option<bool>,
    },

    /// Update a text. If the text has already been sent this will not affect the message.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// Map of values specifying this text message.
        #[arg(long)]
        data: Option<String>,

        /// When the text is deleted, don't delete the attached file. The file being deleted or not does not affect the MMS that was sent. Can only be set to true
        #[arg(long)]
        skip_delete_file: Option<bool>,
    },

    /// Delete a text, canceling it if it has not already been sent. If there is an attached file and skip_delete_file has not been set, the file will be deleted.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

impl Request for TextCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            TextCommands::Create {
                data,
                file_url,
                skip_delete_file,
            } => {
                let create_data = match data {
                    Some(data) => Some(serde_json::from_str(data)?),
                    None => None,
                };

                let request = CreateRequest {
                    data: create_data,
                    file_url: file_url.clone(),
                    skip_delete_file: skip_delete_file.clone(),
                };

                let response_result = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/texts")
                    .header("Access-Token", access_token)
                    .json(&request)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            TextCommands::Update {
                iden,
                data,
                skip_delete_file,
            } => {
                let update_data = match data {
                    Some(data) => Some(serde_json::from_str(data)?),
                    None => None,
                };

                let request = UpdateRequest {
                    data: update_data,
                    skip_delete_file: skip_delete_file.clone(),
                };

                let response_result = reqwest::blocking::Client::new()
                    .post(format!("https://api.pushbullet.com/v2/texts/{}", iden))
                    .header("Access-Token", access_token)
                    .json(&request)
                    .send();

                match response_result {
                    Ok(response) => Ok(response.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            TextCommands::Delete { iden } => {
                let response_result = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/texts/{}", iden))
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
