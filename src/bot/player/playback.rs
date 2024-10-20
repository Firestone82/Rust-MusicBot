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
    pub playing: bool,
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
            playing: false,
            queue: Vec::new(),
            track_handle: None,
            current_track: None
        }
    }

    pub fn add_to_queue(&mut self, video: Track) {
        self.queue.push(video);
    }
    
    pub fn change_playing_state(&mut self, state: bool) {
        if self.playing == state {
            return;
        }

        self.playing = state;
    }
    
    pub fn is_playing(&self) -> bool {
        self.playing
    }
    
    pub fn get_current_track(&self) -> Option<&Track> {
        self.current_track.as_ref()
    }

    pub fn set_handle(&mut self, handle: TrackHandle) {
        self.track_handle = Option::from(handle);
    }
    
    pub fn play_next(&mut self) -> Option<&Track> {
        if self.queue.is_empty() {
            self.playing = false;
            return None;
        }

        if self.playing {
            let current: Option<&Track> = self.get_current_track();

            // if let Some(track) = current {
            //     if let Some(handle) = track.handle.as_ref() {
            //         handle.stop().expect("Unable to stop current playing track");
            //     }
            // }
        }
        
        let track: Track = self.queue.remove(0);
        self.current_track = Option::from(track);
        self.playing = true;
        
        self.current_track.as_ref()
    }
}