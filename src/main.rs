use std::{
    env,
    io::{Error, ErrorKind},
    process::ExitCode,
};

use crate::{
    algorithms::grid_noise,
    image::{generate_pgm, generate_ppm, terrain},
};

mod algorithms;
mod image;

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

    generate_pgm(&grid)?;
    generate_ppm(&grid, &terrain)?;

    Ok(())
}
