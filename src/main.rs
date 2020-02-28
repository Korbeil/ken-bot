extern crate chrono;

use std::env;
use chrono::Utc;
use serenity::{
    model::{channel::Message, gateway::Ready, id::{ChannelId, UserId}},
    prelude::*,
};

fn datetime() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

struct Handler {
    target_channel: ChannelId,
    target_user: UserId,
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if self.target_channel != msg.channel_id {
            return;
        }

        let messages = match msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(msg.id)
        }) {
            Ok(messages) => messages,
            Err(err) => {
                println!("[{}] Error getting channel messages: {:?}", datetime(), err);
                return;
            }
        };

        for message in &messages {
            if message.id == msg.id || self.target_user != message.author.id {
                continue;
            }

            match message.delete(&ctx) {
                Ok(_) => {
                    println!("[{}] Deleted \"{}\" message from \"{}\"", datetime(), message.content, message.author.name)
                },
                Err(err) => {
                    println!("[{}] Error deleting channel messages: {:?}", datetime(), err);
                    return;
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("[{}] {} is connected!", datetime(), ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let channel_id = env::var("DISCORD_CHANNEL_ID").expect("Expected a DISCORD_CHANNEL_ID in the environment");
    let user_id = env::var("DISCORD_USER_ID").expect("Expected a DISCORD_USER_ID in the environment");

    let mut client = Client::new(&token, Handler {
        target_channel: ChannelId(channel_id.parse::<u64>().unwrap()),
        target_user: UserId(user_id.parse::<u64>().unwrap()),
    }).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("[{}] Client error: {:?}", datetime(), why);
    }
}