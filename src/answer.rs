use discord::{
    model::ChannelId,
    Discord
};
use rand::prelude::*;

/// Answer type
enum Answer {
    Text(String),
    Image(String)
}

/// Select an answer to a godwin trigger and send on the channel.
pub fn answer(discord: &Discord, channel: ChannelId) {
    let answers: Vec<Answer> = vec![
        Answer::Text("And it's a godwin point.".to_string()),
        Answer::Text("Somebody call's me?".to_string())
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, answers.len());

    match &answers[index] {
        Answer::Text(mess) => {
            let _ = discord.send_message(channel, &mess, "", false);
        },
        Answer::Image(src) => {
            // TODO
        }
    }
}
