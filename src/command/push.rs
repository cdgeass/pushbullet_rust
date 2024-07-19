use std::{collections::HashMap, error::Error};

use clap::{Args, Subcommand};
use reqwest::blocking::multipart;
use serde_json::json;

use super::{upload_request, Request};

#[derive(Args)]
pub struct PushPaginationArgs {}

#[derive(Subcommand)]
pub enum PushCommands {
    /// Request push history.
    List {
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
    },

    /// Send a push to a device or another person.
    Create {
        /// Type of the push, one of "note", "file", "link".
        #[arg(long, name = "type")]
        t: String,

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
    },

    /// Update a push.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// Marks a push as having been dismissed by the user, will cause any notifications for the push to be hidden if possible.
        #[arg(long)]
        dismissed: Option<bool>,
    },

    /// Delete a push.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },

    /// Delete all pushes belonging to the current user. This call is asynchronous, the pushes will be deleted after the call returns.
    DeleteAll,
}

impl Request for PushCommands {
    fn request(&self, access_token: &str) -> Result<String, Box<dyn Error>> {
        match self {
            PushCommands::List { modified_after, active, cursor, limit, } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .get("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                let mut query: Vec<(String, String)> = vec![];
                if let Some(modified_after) = modified_after {
                    query.push((String::from("modified_after"), modified_after.to_owned()));
                }
                if let Some(active) = active {
                    query.push((String::from("active"), active.to_string()));
                }
                if let Some(cursor) = cursor {
                    query.push((String::from("cursor"), cursor.to_owned()));
                }
                if let Some(limit) = limit {
                    query.push((String::from("limit"), limit.to_string()));
                }
                request_builder = request_builder.query(&query);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
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
            } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post("https://api.pushbullet.com/v2/devices")
                    .header("Access-Token", access_token);

                let mut map = HashMap::new();
                map.insert("type", t.to_owned());
                if let Some(source_device_iden) = source_device_iden {
                    map.insert("source_device_iden", source_device_iden.to_owned());
                }
                if let Some(device_iden) = device_iden {
                    map.insert("device_iden", device_iden.to_owned());
                }
                if let Some(client_iden) = client_iden {
                    map.insert("client_iden", client_iden.to_owned());
                }
                if let Some(channel_tag) = channel_tag {
                    map.insert("channel_tag", channel_tag.to_owned());
                }
                if let Some(email) = email {
                    map.insert("email", email.to_owned());
                }
                if let Some(guid) = guid {
                    map.insert("guid", guid.to_owned());
                }

                match t.as_str() {
                    "note" => {
                        if let Some(title) = title {
                            map.insert("title", title.to_owned());
                        }
                        if let Some(body) = body {
                            map.insert("body", body.to_owned());
                        }
                    }
                    "link" => {
                        if let Some(title) = title {
                            map.insert("title", title.to_owned());
                        }
                        if let Some(body) = body {
                            map.insert("body", body.to_owned());
                        }
                        if let Some(url) = url {
                            map.insert("url", url.to_owned());
                        }
                    }
                    "file" => {
                        let file_name = match file_name {
                            Some(file_name) => file_name,
                            None => todo!(),
                        };

                        match upload_request(
                            access_token.to_owned(),
                            file_name.to_owned(),
                            file_type.to_owned(),
                        ) {
                            Ok(response) => {
                                let upload_url = response.upload_url;
                                let form = multipart::Form::new().file("file", &file_name)?;
                                reqwest::blocking::Client::new()
                                    .post(upload_url)
                                    .header("Access-Token", access_token)
                                    .multipart(form)
                                    .send()?;

                                if let Some(title) = title {
                                    map.insert("title", title.to_owned());
                                }
                                if let Some(body) = body {
                                    map.insert("body", body.to_owned());
                                }
                                if let Some(url) = url {
                                    map.insert("url", url.to_owned());
                                }
                                map.insert("file_name", response.file_name.to_owned());
                                map.insert("file_type", response.file_type.to_owned());
                                map.insert("file_url", response.file_url.to_owned());
                            }
                            Err(_) => todo!(),
                        }
                    }
                    _ => (),
                };
                request_builder = request_builder.json(&map);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::Update { iden, dismissed } => {
                let client = reqwest::blocking::Client::new();

                let mut request_builder = client
                    .post(format!("https://api.pushbullet.com/v2/devices/{}", iden))
                    .header("Access-Token", access_token);

                match dismissed {
                    Some(dismissed) => {
                        request_builder = request_builder.json(&json!({"dismissed": dismissed}));
                    }
                    _ => (),
                }

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::Delete { iden } => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete(format!("https://api.pushbullet.com/v2/pushes/{}", iden))
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
            PushCommands::DeleteAll => {
                let client = reqwest::blocking::Client::new();

                let request_builder = client
                    .delete("https://api.pushbullet.com/v2/pushes")
                    .header("Access-Token", access_token);

                match request_builder.send() {
                    Ok(res) => Ok(res.text()?),
                    Err(e) => Err(Box::new(e)),
                }
            }
        }
    }
}
