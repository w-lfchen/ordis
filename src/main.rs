mod commands;
mod state;

use clap::Parser;
use poise::{
    Framework,
    serenity_prelude::{ClientBuilder, GatewayIntents},
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

    start_bot(&args.bot_token).await?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short('t'), long, env)]
    bot_token: String,
}

/// sets up the bot and then starts it. starting the bot is blocking.
async fn start_bot(bot_token: impl AsRef<str>) -> Result<(), anyhow::Error> {
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
                // don't care if it was already initialized
                let _ = crate::state::set_http(ctx.http.clone());
                Ok(BotData)
            })
        })
        .build();

    ClientBuilder::new(bot_token, intents)
        .framework(framework)
        .await?
        .start()
        .await
        .map_err(Into::into)
}
