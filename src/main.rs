use std::{
    env,
    io::{Error, ErrorKind},
    process::ExitCode,
};

use crate::{
    export::{generate_pgm, generate_ppm},
    noise::grid_noise,
    terrain::colors,
};

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
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(ErrorKind::Other, "Arguments missing"));
    };

    let mut grid: Vec<Vec<f32>> = vec![vec![0.0; 1000]; 1000];

    grid_noise(&mut grid, args[1].parse::<u32>().unwrap());

    // Export to differents formats
    generate_pgm(&grid)?;
    generate_ppm(&grid, &colors)?;

    Ok(())
}
