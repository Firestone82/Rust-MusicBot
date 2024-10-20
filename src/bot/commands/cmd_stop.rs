use crate::bot::{
    checks::channel_checks::check_author_in_same_voice_channel,
    client::{Context, MusicBotError},
    player::playback::Playback
};
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};
use tokio::sync::RwLockWriteGuard;

#[poise::command(
    prefix_command,
    check = "check_author_in_same_voice_channel",
)]
pub async fn stop(ctx: Context<'_>) -> Result<(), MusicBotError> {
    let mut playback: RwLockWriteGuard<Playback> = ctx.data().playback.write().await;

    if let Err(error) = playback.stop_playback(ctx).await {
        let embed: CreateEmbed = CreateEmbed::new()
            .color(Color::RED)
            .title("Failed to stop playback")
            .description(format!("Error: {:?}", error.to_string()));

        println!("Error stopping playback: {:?}", error);
        ctx.send(CreateReply::default().embed(embed)).await?;
    }
    
    drop(playback);
    Ok(())
}
