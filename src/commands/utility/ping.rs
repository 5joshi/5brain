use std::{sync::Arc, time::Instant};

use command_macros::{command, SlashCommand};
use twilight_interactions::command::CreateCommand;
use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    core::Context,
    util::{builder::MessageBuilder, ApplicationCommandExt, MessageExt},
    BotResult,
};

#[derive(CreateCommand, SlashCommand)]
#[command(
    name = "ping",
    help = "Most basic command, generally used to check if the bot is online.\n\
    The displayed latency is the time it takes for the bot \
    to receive a response from discord after sending a message."
)]
#[flags(SKIP_DEFER)]
/// Check if the bot is online
pub struct Ping;

async fn slash_ping(ctx: Arc<Context>, command: Box<ApplicationCommand>) -> BotResult<()> {
    let builder = MessageBuilder::new().content("Pong");
    let start = Instant::now();
    command.callback(&ctx, builder, false).await?;
    let elapsed = (Instant::now() - start).as_millis();
    let response = ctx
        .interaction()
        .response(&command.token)
        .exec()
        .await?
        .model()
        .await?;
    let content = format!(":ping_pong: Pong! ({elapsed}ms)");
    let builder = MessageBuilder::new().content(content);
    response.update(&ctx, &builder).await?;

    Ok(())
}
