use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

/// Generate a pgm file from the values of the given grid.
pub fn generate_pgm(grid: &[Vec<f32>]) -> std::io::Result<()> {
    if grid.is_empty() || grid[0].is_empty() {
        return Ok(());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut file = BufWriter::new(File::create("noise.pgm")?);

    // PGM header
    writeln!(file, "P2")?;
    writeln!(file, "{width} {height}")?;
    writeln!(file, "255")?;

    // Write pixels
    for row in grid {
        for &value in row {
            // Convert from range [-1.0; 1.0] to range [0.0; 255.0]
            let pixel = ((value + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;

            write!(file, "{} ", pixel)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

/// Generate a ppm file from the values of the given grid.
/// `color` maps a noise value in [-1.0; 1.0] to an RGB pixel.
pub fn generate_ppm(grid: &[Vec<f32>], color: impl Fn(f32) -> [u8; 3]) -> std::io::Result<()> {
    if grid.is_empty() || grid[0].is_empty() {
        return Ok(());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut file = BufWriter::new(File::create("noise.ppm")?);

    // PPM header
    writeln!(file, "P3")?;
    writeln!(file, "{width} {height}")?;
    writeln!(file, "255")?;

    // Write pixels
    for row in grid {
        for &value in row {
            let [r, g, b] = color(value);
            write!(file, "{r} {g} {b} ")?;
        }
        writeln!(file)?;
    }

    file.flush()
}
