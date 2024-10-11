use crate::bot::{
    client::{Context, MusicBotError},
    handlers::channel_handler::is_use_in_voice_channel
};
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};

pub async fn check_author_in_voice_channel(ctx: Context<'_>) -> Result<bool, MusicBotError> {
    if let Err(_) = is_use_in_voice_channel(ctx).await {
        let embed = CreateEmbed::new()
            .color(Color::RED)
            .title("No active voice channel")
            .description("You need to be in a voice channel to use this command.");

        ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
        Ok(false)
    } else {
        Ok(true)
    }
}