#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn rectangle() -> Vec<f32> {
    vec![
         // Positions      // Colors        // Texture Coordinates
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // Top Right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // Bottom Right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // Bottom Left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // Top Left
    ]
}