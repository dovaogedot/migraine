use image::Rgb;

use crate::{
    algorithm::kmeans::{Distance, kmeans_elbow, kmeans_pp},
    color::rgb::to_string,
};

#[derive(Clone, Debug, Default)]
pub struct Palette {
    colors: Vec<Rgb<f64>>,
}

impl Palette {
    pub fn new(colors: Vec<Rgb<f64>>) -> Self {
        assert!(colors.len() > 0, "Palette must contain at least one color");
        Palette { colors }
    }

    pub fn colors(&self) -> &[Rgb<f64>] {
        &self.colors
    }

    pub fn closest_to(&self, color: &Rgb<f64>) -> Rgb<f64> {
        self.colors
            .iter()
            .min_by(|&c1, &c2| c1.distance(color).total_cmp(&c2.distance(color)))
            .unwrap_or(&self.colors[0])
            .clone()
    }

    pub fn reduced(&self, palette_size: u32) -> Palette {
        let palette: Vec<Rgb<f64>> = kmeans_pp(palette_size as usize, &self.colors)
            .iter_mut()
            .map(|c| c.centroid())
            .collect();

        Palette::new(palette)
    }

    pub fn reduced_auto(&self, max_colors: Option<u32>) -> Palette {
        let palette: Vec<Rgb<f64>> = kmeans_elbow(&self.colors, max_colors)
            .iter_mut()
            .map(|c| c.centroid())
            .collect();

        Palette::new(palette)
    }

    pub fn to_str(&self) -> String {
        self.colors.iter().map(to_string).collect::<Vec<_>>().join("\n")
    }
}

impl std::fmt::Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut palette_str = String::with_capacity(self.colors.len());

        let mut ansi_codes: Vec<u8> = self.colors.iter().map(|c| rgb_to_ansi256(c.0)).collect();
        ansi_codes.sort();

        for code in ansi_codes {
            let seq = format!("\x1b[48;5;{code}m  ");
            palette_str.push_str(&seq);
        }
        palette_str.push_str("\x1b[0m");

        write!(f, "{}", palette_str)
    }
}

/// Approximates an ANSI 256-color code in O(1) time, ignoring the mutable 0..16 system colors.
/// Assumes input components are in the range `0.0..=1.0`.
pub fn rgb_to_ansi256(rgb: [f64; 3]) -> u8 {
    // Clamp and scale input values from 0.0..1.0 to 0.0..255.0
    let r = rgb[0].clamp(0.0, 1.0) * 255.0;
    let g = rgb[1].clamp(0.0, 1.0) * 255.0;
    let b = rgb[2].clamp(0.0, 1.0) * 255.0;

    // 1. Map directly to the 6x6x6 color cube (Indices 16..=231)
    let map_cube_component = |v: f64| -> (u8, f64) {
        if v < 47.5 {
            (0, 0.0)
        } else if v < 115.0 {
            (1, 95.0)
        } else if v < 155.0 {
            (2, 135.0)
        } else if v < 195.0 {
            (3, 175.0)
        } else if v < 235.0 {
            (4, 215.0)
        } else {
            (5, 255.0)
        }
    };

    let (ri, rv) = map_cube_component(r);
    let (gi, gv) = map_cube_component(g);
    let (bi, bv) = map_cube_component(b);

    let cube_code = 16 + (ri * 36) + (gi * 6) + bi;

    let dr_c = r - rv;
    let dg_c = g - gv;
    let db_c = b - bv;
    let cube_dist = dr_c * dr_c + dg_c * dg_c + db_c * db_c;

    // 2. Map directly to the grayscale ramp (Indices 232..=255)
    let avg = (r + g + b) / 3.0;
    let gray_idx = ((avg - 8.0) / 10.0).round().clamp(0.0, 23.0);
    let gray_v = 8.0 + gray_idx * 10.0;

    let gray_code = 232 + gray_idx as u8;

    let dr_g = r - gray_v;
    let dg_g = g - gray_v;
    let db_g = b - gray_v;
    let gray_dist = dr_g * dr_g + dg_g * dg_g + db_g * db_g;

    // 3. Return whichever candidate is mathematically closer
    if cube_dist <= gray_dist { cube_code } else { gray_code }
}
