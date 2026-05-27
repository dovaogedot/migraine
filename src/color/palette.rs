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
}

impl std::fmt::Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let palette_str = self.colors.iter().map(to_string).collect::<Vec<_>>().join("\n");
        write!(f, "{}", palette_str)
    }
}
