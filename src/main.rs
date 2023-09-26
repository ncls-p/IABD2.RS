use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use serenity::utils::Colour;
use std::env;

#[group]
#[commands(ping, help, infos, github, rust)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let wakeup_channel_id = env::var("WAKEUP_CHANNEL_ID").expect("wakeup channel id");
        let wakeup_channel_id = wakeup_channel_id.parse::<u64>().expect("parse u64");
        let channel = ctx
            .http
            .get_channel(wakeup_channel_id)
            .await
            .expect("Error getting channel");

        let selim_id = env::var("SELIM_ID").expect("selim id");
        let content = format!("T'as vu je suis codé en rust ! <@{}>", selim_id);
        channel
            .id()
            .send_message(&ctx.http, |m| {
                m.content(content);
                m
            })
            .await
            .expect("Error sending message");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
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
    let mut embed = CreateEmbed::default();
    embed
        .title("Commandes")
        .description("Liste des commandes")
        .field("*ping", "Renvoie pong", false)
        .field("*help", "Renvoie ce message", false)
        .field("*infos", "Renvoie des infos sur le bot", false)
        .field("*github", "Renvoie le lien du github", false)
        .field("*rust", "Renvoie le lien du rust", false);

    let cloned_embed = embed.clone();

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(cloned_embed))
        .await?;

    Ok(())
}

#[command]
async fn infos(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed
        .title("Infos")
        .description("Infos sur le bot")
        .field("Créateur", "Ncls", false)
        .field("Langage", "Rust", false)
        .field("Github", "https://github.com/ncls-p/IABD2.RS", false)
        .field("Version", "0.1.0", false)
        .field("Librairie", "Serenity", false)
        .color(Colour::PURPLE)
        .url("https://github.com/ncls-p/IABD2.RS")
        .image("https://cdn.discordapp.com/attachments/1028352036049277060/1156246329736044676/download.png?ex=651445cf&is=6512f44f&hm=19e23e3d7ec03497772c97daccaa35cbb978df8e4c104414c0c2e63c547091f0&");

    let cloned_embed = embed.clone();

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(cloned_embed))
        .await?;

    Ok(())
}

#[command]
async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://github.com/ncls-p/IABD2.RS").await?;

    Ok(())
}

#[command]
async fn rust(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://www.rust-lang.org/").await?;

    Ok(())
}
