use discord::Discord;
use std::env;
use discord::model::Event;

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    ).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);

                if message.content.contains("nazi") {
                    let _ = discord.send_message(message.channel_id, "And it's a godwin point...", "", false);
                }

                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }

}
