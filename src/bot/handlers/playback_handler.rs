use crate::bot::{
    client::{Context, MusicBotError},
    handlers::queue_handler::QueueHandler,
    player::playback::Track
};
use reqwest::Client;
use serenity::all::GuildId;
use songbird::{input::YoutubeDl, tracks::TrackHandle, TrackEvent, {Call, Event}};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub async fn start_queue_playback(ctx: Context<'_>) -> Result<(), MusicBotError> {
    let mut playback= ctx.data().playback.write().await;

    if playback.is_playing() {
        return Ok(());
    } else {
        println!("Starting queue playback");
    }

    let guild_id: GuildId = ctx.guild_id()
        .ok_or_else(|| {
            println!("Could not locate voice channel. Guild ID is none");
            MusicBotError::InternalError("Could not locate voice channel. Guild ID is none".to_owned())
        })?;

    let manager: Arc<Mutex<Call>> = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| {
            println!("Could not locate voice channel. Guild ID is none");
            MusicBotError::InternalError("Could not locate voice channel. Guild ID is none".to_owned())
        })?
        .get_or_insert(guild_id);

    match playback.play_next() {
        Some(next_track) => {
            println!(" - Playing next track: {}", next_track.metadata.title);

            let req_client: &Client = &ctx.data().request_client;

            let mut guard: MutexGuard<Call> = manager.lock().await;
            let track: YoutubeDl = YoutubeDl::new(req_client.clone(), next_track.metadata.url.clone());
            let track_handle: TrackHandle = guard.play(track.into());

            let _ = track_handle.add_event(
                Event::Track(TrackEvent::End),
                QueueHandler::new(
                    manager.clone(),
                    req_client.clone(),
                    ctx.data().playback.clone()
                )
            );

            playback.set_handle(track_handle);
        }

        None => {
            println!(" - No more tracks to play. Stopping playback.");
            playback.change_playing_state(false);
        }
    }

    Ok(())
}

pub async fn add_queue_video(ctx: Context<'_>, video: Track) -> Result<(), MusicBotError> {
    let mut playback= ctx.data().playback.write().await;

    playback.add_to_queue(video);

    drop(playback);
    Ok(())
}