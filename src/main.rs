use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use dotenv::dotenv;
use std::env;
use sui_sdk::SuiClientBuilder;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }

        if msg.content == "!sui" {
            match get_sui_testnet_version().await {
                Ok(version) => {
                    if let Err(e) = msg.channel_id.say(&ctx.http, format!("Sui testnet version: {}", version)).await {
                        error!("Error sending message: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("Error initializing sui client: {:?}", e);
                    if let Err(e) = msg.channel_id.say(&ctx.http, "Failed to get sui client version").await {
                        error!("Error sending message: {:?}", e);
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

// async function to fetch sui testnet
async fn get_sui_testnet_version() -> Result<String, anyhow::Error> {
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    Ok(sui_testnet.api_version().to_string())
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    dotenv().ok(); // Loads the .env file
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
