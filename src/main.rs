mod commands;
mod listeners;
mod state;

use clap::Parser;
use poise::{
    Framework,
    serenity_prelude::{Client, ClientBuilder, GatewayIntents},
};

use crate::{commands::toggle_channel, state::BotData};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Err(e) = dotenvy::dotenv()
        && !e.not_found()
    {
        return Err(anyhow::Error::new(e));
    }
    let args = Args::parse();

    let client = make_client(&args.bot_token).await?;
    // http needs to be set before starting listeners
    // don't care if it was already initialized
    let _ = state::set_http(client.http.clone());

    listeners::register_listeners();

    let mut client = client;
    start_bot(&mut client).await?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short('t'), long, env)]
    bot_token: String,
}

/// sets up the bot and then starts it. starting the bot is blocking.
async fn make_client(bot_token: impl AsRef<str>) -> Result<Client, anyhow::Error> {
    // intents the bot needs
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework: Framework<BotData, anyhow::Error> = Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![toggle_channel()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(BotData)
            })
        })
        .build();

    ClientBuilder::new(bot_token, intents)
        .framework(framework)
        .await
        .map_err(Into::into)
}

async fn start_bot(client: &mut Client) -> Result<(), anyhow::Error> {
    client.start().await.map_err(Into::into)
}
