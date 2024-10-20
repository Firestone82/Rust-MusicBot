use crate::bot::player::playback::Playback;
use async_trait::async_trait;
use lombok::AllArgsConstructor;
use songbird::{
    input::YoutubeDl,
    tracks::TrackHandle,
    {Call, Event, EventContext, EventHandler}
};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, RwLock};

#[derive(AllArgsConstructor, Clone)]
pub struct QueueHandler {
    manager: Arc<Mutex<Call>>,
    req_client: reqwest::Client,
    playback: Arc<RwLock<Playback>>
}

#[async_trait]
impl EventHandler for QueueHandler {
    async fn act(&self, _e: &EventContext<'_>) -> Option<Event> {
        println!("Track has ended. Requesting next song to play.");
        
        let mut playback_guard = self.playback.write().await;
        
        match playback_guard.play_next() {
            Some(next_track) => {
                println!(" - Playing next track: {}", next_track.metadata.title);
                
                let mut guard: MutexGuard<Call> = self.manager.lock().await;
                let track: YoutubeDl = YoutubeDl::new(self.req_client.clone(), next_track.metadata.url.clone());
                let track_handle: TrackHandle = guard.play(track.into());
                
                let _ = track_handle
                    .add_event(Event::Track(songbird::TrackEvent::End), self.clone())
                    .map_err(|e| {
                        println!("Error adding event to track handle: {:?}", e);
                    });
            }
            
            None => {
                println!("- No more tracks to play. Stopping playback.");
                playback_guard.change_playing_state(false);
            }
        }
        
        None
    }
}