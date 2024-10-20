use songbird::tracks::TrackHandle;

#[derive(thiserror::Error, Debug)]
pub enum PlaybackError {
    #[error("Whoops, an internal error occurred: {0}")]
    InternalError(String),
    
    #[error("No tracks in queue")]
    NoTracksInQueue
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub metadata: TrackMetadata
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub channel: String,
    pub url: String,
}

pub struct Playback {
    pub is_playing: bool,
    pub current_track: Option<Track>,
    pub track_handle: Option<TrackHandle>,
    pub queue: Vec<Track>
}

// WTF is this clippy. p_p
impl Default for Playback {
    fn default() -> Self {
        Self::new()
    }
}

impl Playback {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            queue: Vec::new(),
            track_handle: None,
            current_track: None
        }
    }

    pub fn add_to_queue(&mut self, video: Track) {
        self.queue.push(video);
    }
    
    pub fn change_playing_state(&mut self, state: bool) {
        if self.is_playing == state {
            return;
        }

        self.is_playing = state;
    }
    
    pub fn play_next(&mut self) -> Option<&Track> {
        if self.queue.is_empty() {
            self.change_playing_state(false);
            return None;
        }

        if self.is_playing {
            self.stop();
        }
        
        let track: Track = self.queue.remove(0);
        self.current_track = Option::from(track);
        self.change_playing_state(true);

        self.current_track.as_ref()
    }
    
    pub fn stop(&mut self) {
        self.queue.clear();
        self.change_playing_state(false);
        
        if let Some(track_handle) = &self.track_handle {
            if let Err(err) = track_handle.stop() {
                println!("Error stopping track: {:?}", err);
            }
        }
    }
    
}