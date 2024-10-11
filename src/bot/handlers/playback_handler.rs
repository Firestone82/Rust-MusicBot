
#[derive(thiserror::Error, Debug)]
pub enum PlaybackError {
    #[error("Whoops, an internal error occurred: {0}")]
    InternalError(String),
}

pub struct Video {
    pub id: String,
    pub metadata: VideoMetadata,
}

pub struct VideoMetadata {
    pub title: String,
    pub channel: String,
    pub url: String,
}

pub struct Player {
    pub playing: bool,
    pub queue: Vec<Video>
}

impl Player {
    pub fn new() -> Self {
        Self {
            playing: false,
            queue: Vec::new()
        }
    }

    pub fn add_to_queue(&mut self, video: Video) {
        self.queue.push(video);
        println!("Added video to queue: {}", self.queue.last().unwrap().metadata.title);
    }
}