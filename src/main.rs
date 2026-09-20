use clap::Parser;
use poise::{
    Framework,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};

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

#[derive(Debug)]
struct Data;

/// sets up the bot and then starts it. starting the bot is blocking.
async fn start_bot(bot_token: impl AsRef<str>) -> Result<(), anyhow::Error> {
    // intents the bot needs
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework: Framework<Data, anyhow::Error> = Framework::builder()
        .options(poise::FrameworkOptions {
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
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
