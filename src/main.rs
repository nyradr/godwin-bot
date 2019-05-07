use discord::Discord;
use dotenv::dotenv;
use discord::model::Event;

/// Global bot configuration
mod config;
/// Manage answers
mod answer;

fn main() {
    dotenv::dotenv();
    let config = config::Config::from_env();

    let discord = Discord::from_bot_token(config.token()).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);

                if message.content.contains("nazi") {
                    answer::answer(&discord, message.channel_id);
                }
            }
            Ok(event) => {
                println!("{:?}", event);
            }
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }

}
