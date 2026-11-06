use clap::Parser;
use std::path::PathBuf;

use crate::terrain::TerrainType;

#[derive(Parser, Debug)]
#[command(about = "Procedural terrain generator")]
pub struct Args {
    /// Image width in pixels
    #[arg(long, default_value_t = 512, value_parser = clap::value_parser!(u32).range(1..))]
    pub width: u32,

    /// Image height in pixels
    #[arg(long, default_value_t = 512, value_parser = clap::value_parser!(u32).range(1..))]
    pub height: u32,

    /// Random seed
    #[arg(long, default_value_t = 0)]
    pub seed: u32,

    /// Number of noise layers
    #[arg(long, default_value_t = 6, value_parser = clap::value_parser!(u32).range(1..=16))]
    pub octaves: u32,

    /// Size in pixels of the largest features
    #[arg(long, default_value_t = 80.0, value_parser = positive_f32)]
    pub scale: f32,

    /// Amplitude multiplier per octave
    #[arg(long, default_value_t = 0.5, value_parser = positive_f32)]
    pub persistence: f32,

    /// Frequency multiplier per octave
    #[arg(long, default_value_t = 2.0, value_parser = positive_f32)]
    pub lacunarity: f32,

    /// Type of terrain generated. Accept "flat" or "island"
    #[arg(long = "type", value_enum, default_value_t = TerrainType::Flat)]
    pub kind: TerrainType,

    /// The strength used when generating an island terrain. A lower value means a wider island.
    #[arg(long, default_value_t = 1.5, value_parser = positive_f32)]
    pub island_strength: f32,

    /// Output file. Handled extensions: "pgm" and "ppm".
    #[arg(long, default_value = "terrain.ppm", value_parser = parse_output)]
    pub output: Output,
}

fn positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("must be greater than 0".into())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageFormat {
    Pgm,
    Ppm,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub path: PathBuf,
    pub format: ImageFormat,
}

fn parse_output(s: &str) -> Result<Output, String> {
    let path = PathBuf::from(s);

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);

    let format = match ext.as_deref() {
        Some("pgm") => ImageFormat::Pgm,
        Some("ppm") => ImageFormat::Ppm,
        _ => return Err("extension must be .pgm or .ppm".into()),
    };

    Ok(Output { path, format })
}
