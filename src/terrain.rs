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
