use crate::bot::client::{Context, MusicBotError};
use serenity::all::ChannelId;

pub async fn join_user_channel(ctx: Context<'_>) -> Result<(), MusicBotError> {
    let guild_id = ctx.guild_id().ok_or_else(|| {
        println!("Could not locate voice channel. Guild ID is none");
        MusicBotError::InternalError("Could not locate voice channel. Guild ID is none".to_string())
    })?;

    let channel_id: ChannelId = is_use_in_voice_channel(ctx).await?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| {
            return MusicBotError::InternalError("Could not locate voice channel. Songbird manager does not exist".to_string())
        })?;

    if let Err(_) = manager.join(guild_id, channel_id).await {
        return Err(MusicBotError::UnableToJoinVoiceChannelError)
    }

    Ok(())
}

pub async fn is_use_in_voice_channel(ctx: Context<'_>) -> Result<ChannelId, MusicBotError> {
    let channel_id = ctx.guild()
        .and_then(|guild| {
            guild
                .voice_states
                .get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id)
        })
        .ok_or_else(|| {
            return MusicBotError::UserNotInVoiceChannelError
        })?;

    Ok(channel_id)
}