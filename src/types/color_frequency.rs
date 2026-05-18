use crate::{
    algorithm::kmeans::{Centroid, Distance, Same},
    types::Color,
};

/// Holds a color and number of its occurences.
/// 
/// Used to reduce palette colors.
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
