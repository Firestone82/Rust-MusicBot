use serenity::all::{Color, CreateEmbed, Timestamp};

pub async fn create_no_active_voice_embed() -> CreateEmbed {
    CreateEmbed::new()
        .color(Color::DARK_ORANGE)
        .title("No active voice channel")
        .description("You need to be in a voice channel to use this command.")
        .timestamp(Timestamp::now())
}