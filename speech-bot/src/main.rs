use telebot::Bot;
use futures::stream::Stream;
use std::env;
use futures::Future;
// import all available functions
use telebot::functions::*;
mod speech_requests;

fn main() {
    // Create the bot
    let mut bot = Bot::new(&env::var("BOT_TOKEN").unwrap()).update_interval(200);

    // Register a reply command which answers a message
    let handle = bot.new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<empty>".into();
            }

            bot.message(msg.chat.id, text).send()
        })
        .for_each(|_| Ok(()));


    // Register a reply command which answers a message
    let handle2 = bot.new_cmd("/start")
        .and_then(|(bot, msg)| {
            speech_requests::transcription::hello();
            bot.message(msg.chat.id, "Welcome to the Verbio Speech Bot!".into()).send()
        })
        .for_each(|_| Ok(()));

    bot.run_with(handle.join(handle2));
}