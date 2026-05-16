use std::collections::HashMap;

use crate::{
    color::Color,
    kmean::{Centroid, Distance, Same, kmeans},
};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ColorFrequency {
    pub color: Color,
    pub count: u32,
}

impl Distance for ColorFrequency {
    fn distance(a: &Self, b: &Self) -> f64 {
        a.color.distance(&b.color)
    }
}

impl Same for ColorFrequency {
    fn same(a: &Self, b: &Self) -> bool {
        a.color.eq(&b.color)
    }
}

impl Centroid for ColorFrequency {
    fn centroid(cluster: &[Self]) -> Self
    where
        Self: Sized,
    {
        let n = cluster.iter().map(|c| c.count).sum();
        let sum: (u32, u32, u32) = cluster.iter().fold((0, 0, 0), |(r, g, b), f| {
            (
                r + f.color.r as u32 * f.count,
                g + f.color.g as u32 * f.count,
                b + f.color.b as u32 * f.count,
            )
        });

        let r = (sum.0 / n) as u8;
        let g = (sum.1 / n) as u8;
        let b = (sum.2 / n) as u8;

        ColorFrequency {
            color: Color::from((r, g, b)),
            count: n,
        }
    }
}

impl From<(&Color, &u32)> for ColorFrequency {
    fn from(freq: (&Color, &u32)) -> Self {
        ColorFrequency {
            color: *freq.0,
            count: *freq.1,
        }
    }
}

pub fn reduce_palette(colors: &[Color], palette_size: u32) -> Vec<Color> {
    let mut occurences: Vec<ColorFrequency> = colors
        .iter()
        .fold(HashMap::<Color, u32>::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(ColorFrequency::from)
        .collect::<Vec<_>>();

    occurences.sort_by(|a, b| b.count.cmp(&a.count));

    let palette: Vec<Color> = kmeans(palette_size as usize, &occurences)
        .iter()
        .map(|c| ColorFrequency::centroid(c).color)
        .collect();

    palette
}
