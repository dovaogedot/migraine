use std::marker::PhantomData;

use image_lib::Rgb;

use crate::{
    algorithm::{
        kmeans::{Centroid, Distance},
        kmeans_pp::kmeans_pp,
    },
    types::color::to_string,
};

#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct Palette {
    pub colors: Vec<Rgb<f64>>,
    _private: PhantomData<bool>,
}

impl Palette {
    pub fn new(colors: Vec<Rgb<f64>>) -> Self {
        assert!(colors.len() > 0, "Palette must contain at least one color");
        Palette {
            colors,
            _private: PhantomData,
        }
    }

    pub fn closest_to(&self, color: &Rgb<f64>) -> Rgb<f64> {
        self.colors
            .iter()
            .min_by(|&c1, &c2| Distance::distance(c1, color).total_cmp(&Distance::distance(c2, color)))
            .unwrap_or(&self.colors[0])
            .clone()
    }

    pub fn reduced(&self, palette_size: u32) -> Palette {
        let palette: Vec<Rgb<f64>> = kmeans_pp(palette_size as usize, &self.colors)
            .iter()
            .map(|c| Centroid::centroid(c))
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
