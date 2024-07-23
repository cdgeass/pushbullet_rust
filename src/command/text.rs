use clap::Subcommand;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::Request;

#[derive(Subcommand)]
pub enum TextCommands {
    /// Create a new text. The text will automatically be deleted after an hour whether it has been sent or not.
    Create {
        /// The device_iden of the Device to send the message. This device must have SMS Android permissions granted.
        #[arg(long)]
        target_device_iden: Option<String>,

        /// A list of 1 more phone numbers to send this message to. Including than one number sends a group MMS message.
        #[arg(long)]
        address: Option<String>,

        /// The text content of the text message.
        #[arg(long)]
        message: Option<String>,

        /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
        #[arg(long)]
        guid: Option<String>,

        /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
        #[arg(long)]
        status: Option<String>,

        /// The mime type of the file_url being sent with this message. Only required for messages sending a file.
        #[arg(long)]
        file_type: Option<String>,

        /// File download url for an image to send with the text message.
        #[arg(long)]
        file_url: Option<String>,

        /// If set to false, delete the attached file when the Text is deleted.
        #[arg(long)]
        skip_delete_file: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Update a text. If the text has already been sent this will not affect the message.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// The device_iden of the Device to send the message. This device must have SMS Android permissions granted.
        #[arg(long)]
        target_device_iden: Option<String>,

        /// A list of 1 more phone numbers to send this message to. Including than one number sends a group MMS message.
        #[arg(long)]
        address: Option<String>,

        /// The text content of the text message.
        #[arg(long)]
        message: Option<String>,

        /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
        #[arg(long)]
        guid: Option<String>,

        /// Unique identifier optionally set by the client, used to identify a text message to ensure it is not sent multiple times in the case create-text is called for it more than once.
        #[arg(long)]
        status: Option<String>,

        /// The mime type of the file_url being sent with this message. Only required for messages sending a file.
        #[arg(long)]
        file_type: Option<String>,

        /// When the text is deleted, don't delete the attached file. The file being deleted or not does not affect the MMS that was sent. Can only be set to true
        #[arg(long)]
        skip_delete_file: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Delete a text, canceling it if it has not already been sent. If there is an attached file and skip_delete_file has not been set, the file will be deleted.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

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

impl Request for TextCommands {
    fn build_request(
        &self,
        access_token: &str,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        match self {
            TextCommands::Create {
                target_device_iden,
                address,
                message,
                guid,
                status,
                file_type,
                file_url,
                skip_delete_file,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => {
                        let data = Data {
                            target_device_iden: target_device_iden.clone(),
                            address: address.clone(),
                            message: message.clone(),
                            guid: guid.clone(),
                            status: status.clone(),
                            file_type: file_type.clone(),
                        };
                        CreateRequest {
                            data: Some(data),
                            file_url: file_url.clone(),
                            skip_delete_file: skip_delete_file.clone(),
                        }
                    }
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/texts")
                    .json(&request);
                Ok(request_builder)
            }
            TextCommands::Update {
                iden,
                target_device_iden,
                address,
                message,
                guid,
                status,
                file_type,
                skip_delete_file,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(&data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => {
                        let data = Data {
                            target_device_iden: target_device_iden.clone(),
                            address: address.clone(),
                            message: message.clone(),
                            guid: guid.clone(),
                            status: status.clone(),
                            file_type: file_type.clone(),
                        };
                        UpdateRequest {
                            data: Some(data),
                            skip_delete_file: skip_delete_file.clone(),
                        }
                    }
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post(format!("https://api.pushbullet.com/v2/texts/{}", iden))
                    .json(&request);
                Ok(request_builder)
            }
            TextCommands::Delete { iden } => {
                let request_builder = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/texts/{}", iden));
                Ok(request_builder)
            }
        }
    }
}
