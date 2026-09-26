//! command definitions

use poise::CreateReply;

use crate::state::{self, BotData};

type Context<'a> = poise::Context<'a, BotData, anyhow::Error>;

#[poise::command(
    slash_command,
    broadcast_typing,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm"
)]
/// registers (or removes) the current channel for bot messages
pub async fn toggle_channel(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let enabled = state::toggle_channel(ctx.channel_id());
    ctx.send(
        CreateReply::default()
            .content(
                ctx.guild_channel()
                    .await
                    .map(|ch| {
                        format!(
                            "channel \"{}\" is now {}abled",
                            ch.name,
                            if enabled { "en" } else { "dis" }
                        )
                    })
                    .unwrap_or_else(|| {
                        format!(
                            "bot will no{} message you",
                            if enabled { "w" } else { " longer" }
                        )
                    }),
            )
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
