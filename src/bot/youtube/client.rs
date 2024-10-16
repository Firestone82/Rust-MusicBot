use crate::bot::handlers::playback_handler::{Video, VideoMetadata};
use dotenv::var;
use google_youtube3::api::SearchResult;
use google_youtube3::client::NoToken;
use google_youtube3::hyper::client::HttpConnector;
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::YouTube;
use html_escape::decode_html_entities;

#[derive(thiserror::Error, Debug)]
pub enum YoutubeError {
    #[error("Whoops, an internal error occurred: {0}")]
    InternalError(String),

    #[error("Video not found: {0}")]
    VideoNotFound(String),

    #[error("Youtube API error: {0}")]
    ApiError(#[from] google_youtube3::Error),
}

pub struct YoutubeClient {
    api_key: String,
    youtube: YouTube<HttpsConnector<HttpConnector>>,
}

const SINGLE_URI: &str = "https://www.youtube.com/watch?v=";
const PLAYLIST_URI: &str = "https://www.youtube.com/playlist?list=";

impl YoutubeClient {
    pub fn new() -> Self {
        let youtube_token = var("YOUTUBE_TOKEN").expect("Expected a token in the environment.");

        let connector = google_youtube3::hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let youtube_client = google_youtube3::hyper::Client::builder()
            .build(connector);

        Self {
            api_key: youtube_token,
            youtube: YouTube::new(youtube_client, NoToken),
        }
    }

    pub async fn search_video(&self, url: String) -> Result<Video, YoutubeError> {
        let request = self
            .youtube
            .search()
            .list(&vec![
                String::from("snippet")
            ])
            .q(&url)
            .param("key", &self.api_key)
            .add_type("video")
            .max_results(1);

        let (_, list) = request.doit().await.map_err(|error| {
            println!("{}", error);
            YoutubeError::ApiError(error)
        })?;

        let results: Vec<SearchResult> = list.items.ok_or_else(|| {
            YoutubeError::InternalError("No video found".to_string())
        })?;

        if results.is_empty() {
            return Err(YoutubeError::VideoNotFound(url));
        }

        let result: &SearchResult = results
            .first()
            .ok_or_else(|| YoutubeError::InternalError("No video found".to_string()))?;

        let video_id: Option<String> = result
            .id
            .as_ref()
            .and_then(|resource_id| resource_id.video_id.clone());

        let metadata: Option<VideoMetadata> = result.snippet.as_ref().and_then(|snippet| {
            let title: Option<&String> = snippet.title.as_ref();
            let channel: Option<&String> = snippet.channel_title.as_ref();

            match (video_id.clone(), title, channel) {
                (Some(video_id), Some(title), Some(channel)) => Some(
                    VideoMetadata {
                        title: decode_html_entities(title).to_string(),
                        channel: decode_html_entities(channel).to_string(),
                        url: format!("{SINGLE_URI}{video_id}"),
                    }
                ),
                _ => None,
            }
        });

        match(video_id, metadata) {
            (Some(id), Some(metadata)) => Ok(
                Video {
                    id,
                    metadata
                }
            ),
            _ => Err(YoutubeError::InternalError("Failed to parse video".to_string()))
        }
    }
}