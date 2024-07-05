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
    #[arg(short, long)]
    pub cursor: Option<String>,

    /// You can specify a limit parameter that return a list of objects to get a smaller number of objects on each page.
    #[arg(short, long, default_value = "500")]
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
}

#[derive(Subcommand)]
pub enum ChatCommands {
    /// Get a list of chats belonging to the current user.
    List(PaginationArgs),

    /// Create a chat with another user or email address if one does not already exist.
    Create {
        /// Email of person to create chat with (does not have to be a Pushbullet user)
        email: String,
    },

    /// Update existing chat object.
    Update {
        /// Unique identifier for this object
        iden: String,

        /// true to mute the grant, false to unmute it
        #[arg(short, long)]
        muted: bool,
    },

    /// Delete a chat object.
    Delete {
        /// Unique identifier for this object
        iden: String,
    },
}
