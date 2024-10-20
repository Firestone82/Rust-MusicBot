use crate::bot::client::{Context, MusicBotError};
use serenity::all::{ChannelId, UserId};

pub async fn join_user_channel(ctx: Context<'_>) -> Result<(), MusicBotError> {
    let guild_id = ctx.guild_id().ok_or_else(|| {
        println!("Could not locate voice channel. Guild ID is none");
        MusicBotError::InternalError("Could not locate voice channel. Guild ID is none".to_owned())
    })?;

    match get_user_voice_channel(ctx, &ctx.author().id).await {
        Some(user_channel) => {
            let manager = songbird::get(ctx.serenity_context())
                .await
                .ok_or_else(|| {
                    MusicBotError::InternalError("Could not locate voice channel. Songbird manager does not exist".to_owned())
                })?;

            if (manager.join(guild_id, user_channel).await).is_err() {
                return Err(MusicBotError::UnableToJoinVoiceChannelError)
            }
        }

        None => {
            return Err(MusicBotError::UserNotInVoiceChannelError)
        }
    }

    Ok(())
}

pub async fn get_user_voice_channel(ctx: Context<'_>, user_id: &UserId) -> Option<ChannelId> {
    ctx
        .guild()
        .as_ref()
        .and_then(|guild| guild.voice_states.get(user_id))
        .and_then(|voice_state| voice_state.channel_id)
}