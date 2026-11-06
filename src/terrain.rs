use clap::ValueEnum;

use crate::config::Args;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum TerrainType {
    /// Plain terrain
    Flat,
    /// Terrain fading to ocean at the edges
    Island,
}

/// Simple terrain palette.
pub fn colors(value: f32) -> [u8; 3] {
    match (value + 1.0) * 0.5 {
        t if t < 0.25 => [0, 50, 135],    // deep water
        t if t < 0.35 => [30, 80, 170],   // water
        t if t < 0.42 => [220, 200, 140], // sand
        t if t < 0.65 => [20, 190, 45],   // plain
        t if t < 0.8 => [0, 150, 20],     // hills
        _ => [120, 110, 100],             // rock
    }
}

pub fn apply_island_type(grid: &mut [Vec<f32>], args: &Args) {
    let center_x = args.width as f32 / 2.0;
    let center_y = args.height as f32 / 2.0;

    for y in 0..args.height {
        for x in 0..args.width {
            let distance_x = (x as f32 - center_x) / center_x;
            let distance_y = (y as f32 - center_y) / center_y;
            let distance = distance_x.hypot(distance_y);

            let x = x as usize;
            let y = y as usize;

            grid[y][x] = (grid[y][x] - distance * args.island_strength).clamp(-1.0, 1.0);
        }
    }
}
