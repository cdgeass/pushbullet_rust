use clap::Parser;
use command::{read_access_token, set_access_token, Cli, Commands::*, Request};

mod command;

fn main() {
    let cli = Cli::parse();

    if let AccessToken { access_token } = cli.command {
        if let Err(e) = set_access_token(&access_token) {
            panic!("Set access token error: {e:?}");
        }
    } else {
        let access_token = read_access_token().unwrap_or_else(|e| {
            panic!("{e:?}");
        });

        match cli.command {
            Chat(chat_commands) => match chat_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            _ => (),
        }
    }
}
