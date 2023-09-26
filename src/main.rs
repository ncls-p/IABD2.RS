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
        if let (Ok(wakeup_channel_id), Ok(selim_id)) =
            (env::var("WAKEUP_CHANNEL_ID"), env::var("SELIM_ID"))
        {
            if let (Ok(wakeup_channel_id), Ok(selim_id)) =
                (wakeup_channel_id.parse::<u64>(), selim_id.parse::<u64>())
            {
                if let Ok(channel) = ctx.http.get_channel(wakeup_channel_id).await {
                    let content = format!("T'as vu je suis codé en rust ! <@{}>", selim_id);
                    if let Err(err) = channel
                        .id()
                        .send_message(&ctx.http, |m| {
                            m.content(content);
                            m
                        })
                        .await
                    {
                        eprintln!("Error sending message: {:?}", err);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("*")) // set the bot's prefix to "*"
        .group(&GENERAL_GROUP);

    if let Ok(token) = env::var("DISCORD_TOKEN") {
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            eprintln!("An error occurred while running the client: {:?}", why);
        }
    } else {
        eprintln!("DISCORD_TOKEN not found in environment variables");
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start_time = msg.timestamp.timestamp();
    let mut message = msg.reply(ctx, "Pinging...").await?;

    let end_time = message.timestamp.timestamp();
    let latency = end_time - start_time;

    if let Err(err) = message
        .edit(ctx, |m| m.content(format!("Pong! Latency: {}ms", latency)))
        .await
    {
        eprintln!("Error editing message: {:?}", err);
    }

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

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed.clone()))
        .await
    {
        eprintln!("Error sending help message: {:?}", err);
    }

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

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed.clone()))
        .await
    {
        eprintln!("Error sending info message: {:?}", err);
    }

    Ok(())
}

#[command]
async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(err) = msg.reply(ctx, "https://github.com/ncls-p/IABD2.RS").await {
        eprintln!("Error sending github link: {:?}", err);
    }

    Ok(())
}

#[command]
async fn rust(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(err) = msg.reply(ctx, "https://www.rust-lang.org/").await {
        eprintln!("Error sending rust link: {:?}", err);
    }

    Ok(())
}
