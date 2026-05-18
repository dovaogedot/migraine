use std::marker::PhantomData;

use crate::types::Color;

#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct Palette {
    pub colors: Vec<Color>,
    _private: PhantomData<bool>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        assert!(colors.len() > 0, "Palette must contain at least one color");
        Palette { colors, _private: PhantomData }
    }

    pub fn closest_to(&self, color: &Color) -> Color {
        self.colors
            .iter()
            .min_by(|c1, c2| c1.distance(&color).total_cmp(&c2.distance(&color)))
            .unwrap_or(&self.colors[0])
            .clone()
    }
}

impl std::fmt::Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let palette_str = self
            .colors
            .iter()
            .map(|c| format!("{} {}", c.to_hex_string(), c.to_rgb_string()))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", palette_str)
    }
}
