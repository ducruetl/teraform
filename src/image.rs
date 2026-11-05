use std::fs::File;
use std::io::Write;

pub fn generate_pgm(grid: Vec<Vec<f32>>) -> std::io::Result<()> {
    if grid.is_empty() || grid[0].is_empty() {
        return Ok(());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut file = File::create("noise.pgm")?;

    // PGM header
    writeln!(file, "P2")?;
    writeln!(file, "{width} {height}")?;
    writeln!(file, "255")?;

    // Write pixels
    for row in grid {
        for value in row {
            // Convert from range [-1.0; 1.0] to range [0.0; 255.0]
            let pixel = ((value + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;

            write!(file, "{} ", pixel)?;
        }
        writeln!(file)?;
    }

    Ok(())
}
