use crate::bot::{
    checks::channel_checks::check_author_in_same_voice_channel,
    client::{Context, MusicBotError},
    handlers::channel_handler,
    handlers::playback_handler,
    player::playback::Track,
    youtube::client::{YoutubeClient, YoutubeError}
};
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};

#[poise::command(
    prefix_command,
    check = "check_author_in_same_voice_channel",
)]
pub async fn play(ctx: Context<'_>, youtube_url: String) -> Result<(), MusicBotError> {
    if let Err(error) = channel_handler::join_user_channel(ctx).await {
        let embed: CreateEmbed = CreateEmbed::new()
            .color(Color::RED)
            .title("Failed to join voice channel")
            .description(format!("Error: {:?}", error.to_string()));

        println!("Error joining voice channel: {:?}", error);
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    let youtube_client: &YoutubeClient = &ctx.data().youtube_client;
    let result: Result<Track, YoutubeError> = youtube_client.search_video(youtube_url).await;

    match result {
        Ok(track) => {
            playback_handler::add_queue_video(ctx, track).await?;
            playback_handler::start_queue_playback(ctx).await?;
        }

        Err(error) => {
            let embed: CreateEmbed = CreateEmbed::new()
                .color(Color::RED)
                .title("Failed to search for video")
                .description(format!("Error: {:?}", error.to_string()));

            println!("Error searching for video: {:?}", error);
            ctx.send(CreateReply::default().embed(embed)).await?;
        }
    }

    Ok(())
}
