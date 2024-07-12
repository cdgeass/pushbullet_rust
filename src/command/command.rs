use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct PaginationArgs {
    /// When listing objects, if you receive a cursor in the response, it means the results are on multiple pages. To request the next page of results, use this cursor as the parameter cursor in the next request.
    #[arg(long)]
    pub cursor: Option<String>,

    /// You can specify a limit parameter that return a list of objects to get a smaller number of objects on each page.
    #[arg(long, default_value = "500")]
    pub limit: Option<i32>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// To access the API you'll need an access token so the server knows who you are.
    AccessToken {
        /// You can get one from your Account Settings page.
        access_token: String,
    },

    /// Chats are created whenever you send a message to someone or a receive a message from them and there is no existing chat between you and the other user.
    #[command(subcommand)]
    Chat(ChatCommands),

    #[command(subcommand)]
    Device(DeviceCommands),

    /// A Push.
    #[command(subcommand)]
    Push(PushCommands),
}

#[derive(Subcommand)]
pub enum ChatCommands {
    /// Get a list of chats belonging to the current user.
    List(PaginationArgs),

    /// Create a chat with another user or email address if one does not already exist.
    Create {
        /// Email of person to create chat with (does not have to be a Pushbullet user)
        #[arg(long)]
        email: String,
    },

    /// Update existing chat object.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(long)]
        muted: Option<bool>,
    },

    /// Delete a chat object.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

#[derive(Subcommand)]
pub enum DeviceCommands {
    /// Get a list of devices belonging to the current user.
    List(PaginationArgs),

    /// Create a new device.
    Create {
        /// Name to use when displaying the device
        #[arg(long)]
        nickname: Option<String>,

        /// Model of the device
        #[arg(long)]
        model: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        manufacturer: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        push_token: Option<String>,

        /// Version of the Pushbullet application installed on the device
        #[arg(long)]
        app_version: Option<i32>,

        /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"S
        #[arg(long)]
        icon: Option<String>,

        /// true if the devices has SMS capability, currently only true for type="android" devices
        #[arg(long)]
        has_sms: Option<String>,

        /// reques with body like {"app_version":8623,"manufacturer":"Apple","model":"iPhone 5s (GSM)","nickname":"Elon Musk's iPhone","push_token":"production:f73be0ee7877c8c7fa69b1468cde764f"}'
        #[arg(long)]
        body: Option<String>,
    },

    /// Update an existing device.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// Name to use when displaying the device
        #[arg(long)]
        nickname: Option<String>,

        /// Model of the device
        #[arg(long)]
        model: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        manufacturer: Option<String>,

        /// Platform-specific push token. If you are making your own device, leave this blank and you can listen for events on the Realtime Event Stream.
        #[arg(long)]
        push_token: Option<String>,

        /// Version of the Pushbullet application installed on the device
        #[arg(long)]
        app_version: Option<i32>,

        /// Icon to use for this device, can be an arbitrary string. Commonly used values are: "desktop", "browser", "website", "laptop", "tablet", "phone", "watch", "system"S
        #[arg(long)]
        icon: Option<String>,

        /// true if the devices has SMS capability, currently only true for type="android" devices
        #[arg(long)]
        has_sms: Option<String>,
    },

    /// Delete a device.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}

#[derive(Args)]
pub struct PushPaginationArgs {
    /// Request pushes modified after this timestamp
    #[arg(long)]
    pub modified_after: Option<String>,

    /// Don't return deleted pushes
    #[arg(long)]
    pub active: Option<bool>,

    /// When listing objects, if you receive a cursor in the response, it means the results are on multiple pages. To request the next page of results, use this cursor as the parameter cursor in the next request.
    #[arg(long)]
    pub cursor: Option<String>,

    /// You can specify a limit parameter that return a list of objects to get a smaller number of objects on each page.
    #[arg(long, default_value = "500")]
    pub limit: Option<i32>,
}

#[derive(Subcommand)]
pub enum PushCommands {
    /// Request push history.
    List(PushPaginationArgs),

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
