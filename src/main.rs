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
            Device(device_commands) => match device_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            Push(push_commands) => match push_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            Channel(channel_commands) => match channel_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            Subscription(subscription_commands) => match subscription_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            Text(text_commands) => match text_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            User(user_commands) => match user_commands.request(&access_token) {
                Ok(res) => println!("{res}"),
                Err(e) => panic!("{e:?}"),
            },
            _ => (),
        }
    }
}
