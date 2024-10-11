use crate::bot::{
    checks::author_in_voice_channel::check_author_in_voice_channel,
    client::{Context, MusicBotError},
    handlers::{channel_handler, playback_handler::Video},
    youtube::client::{YoutubeClient, YoutubeError}
};
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};

#[poise::command(
    prefix_command,
    check = "check_author_in_voice_channel",
)]
pub async fn play(
    ctx: Context<'_>,
    youtube_url: String,
) -> Result<(), MusicBotError> {
    if let Err(error) = channel_handler::join_user_channel(ctx).await {
        let embed: CreateEmbed = CreateEmbed::new()
            .color(Color::RED)
            .title("Failed to join voice channel")
            .description(format!("Error: {:?}", error.to_string()));

        println!("Error joining voice channel: {:?}", error);
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    let youtube_client: &YoutubeClient = &ctx.data().youtube_client;
    let result: Result<Video, YoutubeError> = youtube_client.search_video(youtube_url).await;

    match result {
        Ok(video) => {
            let embed: CreateEmbed = CreateEmbed::new()
                .color(Color::DARK_GREEN)
                .title("Video found")
                .description(format!("Video: {:?}", video.metadata.title));

            println!("Video found: {:?}", video.metadata.title);
            ctx.send(CreateReply::default().embed(embed)).await?;

            let mut guard= ctx.data().player.write().await;
            guard.add_to_queue(video);
            drop(guard);
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
