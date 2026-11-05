fn hash(seed: u32, x: i32, y: i32) -> f32 {
    let mut h = seed;

    h ^= (x as u32).wrapping_mul(374761393);
    h = h.rotate_left(13);
    h ^= (y as u32).wrapping_mul(668265263);

    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    h = h.wrapping_mul(2246822519);
    h ^= h >> 13;

    (h as f64 / u32::MAX as f64 * 2.0 - 1.0) as f32
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn noise(seed: u32, x: f32, y: f32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;

    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let tx = fade(x - x0 as f32);
    let ty = fade(y - y0 as f32);

    // Random values at lattice corners
    let v00 = hash(seed, x0, y0);
    let v10 = hash(seed, x1, y0);
    let v01 = hash(seed, x0, y1);
    let v11 = hash(seed, x1, y1);

    // Interpolate horizontally
    let a = lerp(v00, v10, tx);
    let b = lerp(v01, v11, tx);

    // Interpolate vertically
    lerp(a, b, ty)
}

pub fn grid_noise(grid: &mut Vec<Vec<f32>>, seed: u32) {
    let height = grid.len();
    if height == 0 {
        return;
    }

    let width = grid[0].len();
    let scale = 0.1;

    for y in 0..height {
        for x in 0..width {
            grid[y][x] = noise(seed, x as f32 * scale, y as f32 * scale);
        }
    }
}
