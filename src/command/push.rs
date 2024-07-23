use std::error::Error;

use clap::{Args, Subcommand};
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::{upload, upload_request, Request};

#[derive(Args)]
pub struct PaginationArgs {
    /// Request pushes modified after this timestamp
    #[arg(long)]
    modified_after: Option<String>,

    /// Don't return deleted pushes
    #[arg(long)]
    active: Option<bool>,

    /// When listing objects, if you receive a cursor in the response, it means the results are on multiple pages. To request the next page of results, use this cursor as the parameter cursor in the next request.
    #[arg(long)]
    cursor: Option<String>,

    /// You can specify a limit parameter that return a list of objects to get a smaller number of objects on each page.
    #[arg(long, default_value = "500")]
    limit: Option<i32>,
}

impl PaginationArgs {
    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query: Vec<(String, String)> = vec![];
        if let Some(modified_after) = &self.modified_after {
            query.push((String::from("modified_after"), modified_after.to_owned()));
        }
        if let Some(active) = self.active {
            query.push((String::from("active"), active.to_string()));
        }
        if let Some(cursor) = &self.cursor {
            query.push((String::from("cursor"), cursor.to_owned()));
        }
        if let Some(limit) = self.limit {
            query.push((String::from("limit"), limit.to_string()));
        }
        query
    }
}

#[derive(Subcommand)]
pub enum PushCommands {
    /// Request push history.
    List(PaginationArgs),

    /// Send a push to a device or another person.
    Create {
        /// Type of the push, one of "note", "file", "link".
        #[arg(long, name = "type")]
        t: Option<String>,

        /// Title of the push, used for all types of pushes
        #[arg(long)]
        title: Option<String>,

        /// Body of the push, used for all types of pushes
        #[arg(long)]
        body: Option<String>,

        /// URL field, used for type="link" pushes
        #[arg(long)]
        url: Option<String>,

        /// File name, used for type="file" pushes
        #[arg(long)]
        file_name: Option<String>,

        /// File mime type, used for type="file" pushes
        #[arg(long)]
        file_type: Option<String>,

        /// File download url, used for type="file" pushes
        #[arg(long)]
        file_url: Option<String>,

        /// Device iden of the sending device. Optional.
        #[arg(long)]
        source_device_iden: Option<String>,

        /// Device iden of the target device, if sending to a single device. Appears as target_device_iden on the push.
        #[arg(long)]
        device_iden: Option<String>,

        /// Client iden of the target client, sends a push to all users who have granted access to this client. The current user must own this client.
        #[arg(long)]
        client_iden: Option<String>,

        /// Channel tag of the target channel, sends a push to all people who are subscribed to this channel. The current user must own this channel.
        #[arg(long)]
        channel_tag: Option<String>,

        /// Email address to send the push to. If there is a pushbullet user with this address, they get a push, otherwise they get an email.
        #[arg(long)]
        email: Option<String>,

        /// Unique identifier set by the client, used to identify a push in case you receive it from /v2/everything before the call to /v2/pushes has completed. This should be a unique value. Pushes with guid set are mostly idempotent, meaning that sending another push with the same guid is unlikely to create another push (it will return the previously created push).
        #[arg(long)]
        guid: Option<String>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Update a push.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// Marks a push as having been dismissed by the user, will cause any notifications for the push to be hidden if possible.
        #[arg(long)]
        dismissed: Option<bool>,

        #[arg(long)]
        data_binary: Option<String>,
    },

    /// Delete a push.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },

    /// Delete all pushes belonging to the current user. This call is asynchronous, the pushes will be deleted after the call returns.
    DeleteAll,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Type of the push, one of "note", "file", "link".
    #[serde(rename = "type")]
    t: Option<String>,

    /// Title of the push, used for all types of pushes
    title: Option<String>,

    /// Body of the push, used for all types of pushes
    body: Option<String>,

    /// URL field, used for type="link" pushes
    url: Option<String>,

    /// File name, used for type="file" pushes
    file_name: Option<String>,

    /// File mime type, used for type="file" pushes
    file_type: Option<String>,

    /// File download url, used for type="file" pushes
    file_url: Option<String>,

    /// Device iden of the sending device. Optional.
    source_device_iden: Option<String>,

    /// Device iden of the target device, if sending to a single device. Appears as target_device_iden on the push.
    device_iden: Option<String>,

    /// Client iden of the target client, sends a push to all users who have granted access to this client. The current user must own this client.
    client_iden: Option<String>,

    /// Channel tag of the target channel, sends a push to all people who are subscribed to this channel. The current user must own this channel.
    channel_tag: Option<String>,

    /// Email address to send the push to. If there is a pushbullet user with this address, they get a push, otherwise they get an email.
    email: Option<String>,

    /// Unique identifier set by the client, used to identify a push in case you receive it from /v2/everything before the call to /v2/pushes has completed. This should be a unique value. Pushes with guid set are mostly idempotent, meaning that sending another push with the same guid is unlikely to create another push (it will return the previously created push).
    guid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// Marks a push as having been dismissed by the user, will cause any notifications for the push to be hidden if possible.
    dismissed: Option<bool>,
}

impl Request for PushCommands {
    fn build_request(&self, access_token: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        match self {
            PushCommands::List(args) => {
                let request_builder = reqwest::blocking::Client::new()
                    .get("https://api.pushbullet.com/v2/pushes")
                    .query(&args.to_query());
                Ok(request_builder)
            }
            PushCommands::Create {
                t,
                title,
                body,
                url,
                file_name,
                file_type,
                file_url,
                source_device_iden,
                device_iden,
                client_iden,
                channel_tag,
                email,
                guid,
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
                        let mut file_name = file_name.clone();
                        let mut file_type = file_type.clone();
                        let mut file_url = file_url.clone();

                        let file_name_clone = file_name.clone();
                        if let Some(t) = t {
                            if t == "file" && file_name_clone.is_some() {
                                match upload_request(
                                    access_token,
                                    file_name.unwrap(),
                                    file_type.to_owned(),
                                ) {
                                    Ok(response) => {
                                        let upload_url = response.upload_url;
                                        match upload(
                                            access_token,
                                            file_name_clone.unwrap().as_str(),
                                            &upload_url,
                                        ) {
                                            Ok(_) => {
                                                file_name = Some(response.file_name);
                                                file_type = Some(response.file_type);
                                                file_url = Some(response.file_url);
                                            }
                                            Err(error) => {
                                                return Err(error);
                                            }
                                        };
                                    }
                                    Err(error) => {
                                        return Err(error);
                                    }
                                };
                            }
                        }

                        CreateRequest {
                            t: t.clone(),
                            title: title.clone(),
                            body: body.clone(),
                            url: url.clone(),
                            file_name: file_name,
                            file_type: file_type,
                            file_url: file_url,
                            source_device_iden: source_device_iden.clone(),
                            device_iden: device_iden.clone(),
                            client_iden: client_iden.clone(),
                            channel_tag: channel_tag.clone(),
                            email: email.clone(),
                            guid: guid.clone(),
                        }
                    }
                };

                let request_builder = reqwest::blocking::Client::new()
                    .post("https://api.pushbullet.com/v2/pushes")
                    .json(&request);
                Ok(request_builder)
            }
            PushCommands::Update {
                iden,
                dismissed,
                data_binary,
            } => {
                let request = match data_binary {
                    Some(data_binary) => match serde_json::from_str(data_binary) {
                        Ok(request) => request,
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    },
                    None => UpdateRequest {
                        dismissed: dismissed.clone(),
                    },
                };
                let request_builder = reqwest::blocking::Client::new()
                    .post(format!("https://api.pushbullet.com/v2/pushes/{}", iden))
                    .json(&request);
                Ok(request_builder)
            }
            PushCommands::Delete { iden } => {
                let request_builder = reqwest::blocking::Client::new()
                    .delete(format!("https://api.pushbullet.com/v2/pushes/{}", iden));
                Ok(request_builder)
            }
            PushCommands::DeleteAll => {
                let request_builder =
                    reqwest::blocking::Client::new().delete("https://api.pushbullet.com/v2/pushes");
                Ok(request_builder)
            }
        }
    }
}
