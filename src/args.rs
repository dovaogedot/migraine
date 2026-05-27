use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct MigraineArgs {
    /// Path to the upscaled version of the pixel art
    pub path: PathBuf,

    /// How many pixels in source image correspond to one pixel in original pixel art, can be fractional
    #[arg(short, long)]
    pub scale: Option<f64>,

    /// Original width of the pixel art
    #[arg(short, long)]
    pub width: Option<u32>,

    /// Original height of the pixel art
    #[arg(long)]
    pub height: Option<u32>,

    /// Explains what's happenning
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Reduce palette to that amount of colors
    #[arg(short, long)]
    pub colors: Option<u32>,

    /// Pick best amount of palette colors up to this number
    #[arg(short, long)]
    pub max_colors: Option<u32>,
}
