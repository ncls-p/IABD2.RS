use std::env;

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::*;

#[group]
#[commands(ping, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let channel = ctx
            .http
            .get_channel(1073264586653978697)
            .await
            .expect("Error getting channel");

        channel
            .id()
            .send_message(&ctx.http, |m| {
                m.content("T'as vu je suis codé en rust ! <@296305445218287618>");
                m
            })
            .await
            .expect("Error sending message");
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("*")) // set the bot's prefix to "*"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start_time = msg.timestamp.timestamp();
    let mut message = msg.reply(ctx, "Pinging...").await?;

    let end_time = message.timestamp.timestamp();
    let latency = end_time - start_time;

    message
        .edit(ctx, |m| m.content(format!("Pong! Latency: {}ms", latency)))
        .await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    // Create a new Embed
    let mut embed = CreateEmbed::default();
    embed
        .title("Commandes")
        .description("Liste des commandes")
        .field("*ping", "Renvoie pong", false)
        .field("*help", "Renvoie ce message", false)
        .field("*github", "Renvoie le lien du github", false)
        .field("*rust", "Renvoie le lien du rust", false);

    // Clone the embed
    let cloned_embed = embed.clone();

    // Send it
    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(cloned_embed))
        .await?;

    Ok(())
}
