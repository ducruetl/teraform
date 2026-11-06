use std::{io::Error, process::ExitCode};

use clap::Parser;

use crate::{
    config::{Args, ImageFormat},
    export::{generate_pgm, generate_ppm},
    noise::grid_noise,
    terrain::{TerrainType, apply_island_type, colors},
};

mod config;
mod export;
mod noise;
mod terrain;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    let mut grid: Vec<Vec<f32>> = vec![vec![0.0; args.width as usize]; args.height as usize];

    grid_noise(&mut grid, &args);

    match args.kind {
        TerrainType::Island => apply_island_type(&mut grid, &args),
        _ => {}
    }

    match args.output.format {
        ImageFormat::Pgm => generate_pgm(&grid, &args.output.path)?,
        ImageFormat::Ppm => generate_ppm(&grid, &colors, &args.output.path)?,
    };

    Ok(())
}
