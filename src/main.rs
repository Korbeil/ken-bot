use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready, id::{ChannelId, UserId}},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let channel_id = env::var("DISCORD_CHANNEL_ID").expect("Expected a DISCORD_CHANNEL_ID in the environment");
        let target_channel = ChannelId(channel_id.parse::<u64>().unwrap());

        if target_channel != msg.channel_id {
            return;
        }

        let user_id = env::var("DISCORD_USER_ID").expect("Expected a DISCORD_USER_ID in the environment");
        let target_user = UserId(user_id.parse::<u64>().unwrap());
        
        let messages = match msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(msg.id)
        }) {
            Ok(messages) => messages,
            Err(err) => {
                println!("Error getting channel messages: {:?}", err);
                return;
            }
        };

        for message in &messages {
            if message.id == msg.id || target_user != message.author.id {
                continue;
            }

            match message.delete(&ctx) {
                Ok(ok) => ok,
                Err(err) => {
                    println!("Error deleting channel messages: {:?}", err);
                    return;
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}