use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "wav/"]
pub struct Asset;
