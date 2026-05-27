use std::ops::{Mul, Sub};

use image::Rgb;

use crate::algorithm::kmeans::{Centroid, Distance, Same};

impl Centroid for Rgb<f64> {
    fn centroid(cluster: &[&Self]) -> Self {
        let (r, g, b) = cluster
            .iter()
            .map(|x| x.0)
            .fold((0.0, 0.0, 0.0), |(r_sum, g_sum, b_sum), [r, g, b]| {
                (r_sum + r, g_sum + g, b_sum + b)
            });
        let total = cluster.len() as f64;
        [r / total, g / total, b / total].into()
    }
}

impl Distance for Rgb<f64> {
    fn distance(&self, other: &Self) -> f64 {
        let r = self[0] - other[0];
        let g = self[1] - other[1];
        let b = self[2] - other[2];
        ((r * r) * 0.3 + (g * g) * 0.59 + (b * b) * 0.11).sqrt()
    }
}

impl Distance for Rgb<f32> {
    fn distance(&self, other: &Self) -> f64 {
        let r = (self[0] - other[0]) as f64;
        let g = (self[1] - other[1]) as f64;
        let b = (self[2] - other[2]) as f64;
        ((r * r) * 0.3 + (g * g) * 0.59 + (b * b) * 0.11).sqrt()
    }
}

impl Same for Rgb<f64> {
    fn same(&self, other: &Self) -> bool {
        self[0].sub(other[0]).abs() < f64::EPSILON
            && self[1].sub(other[1]).abs() < f64::EPSILON
            && self[2].sub(other[2]).abs() < f64::EPSILON
    }
}

pub fn to_rgb_string(color: &Rgb<f64>) -> String {
    format!(
        "({:3}, {:3}, {:3})",
        color[0].mul(255.0).round(),
        color[1].mul(255.0).round(),
        color[2].mul(255.0).round(),
    )
}

pub fn to_hex_string(color: &Rgb<f64>) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        color[0].mul(255.0).round() as u8,
        color[1].mul(255.0).round() as u8,
        color[2].mul(255.0).round() as u8,
    )
}

pub fn to_string(color: &Rgb<f64>) -> String {
    format!("{} {}", to_hex_string(color), to_rgb_string(color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_string() {
        let color: Rgb<f64> = [1.0, 0.5, 0.25].into();
        let rgb_string = to_rgb_string(&color);
        assert_eq!(rgb_string, "(255, 128,  64)");
    }

    #[test]
    fn hex_string() {
        let color: Rgb<f64> = [1.0, 0.5, 0.25].into();
        let hex_string = to_hex_string(&color);
        assert_eq!(hex_string, "#ff8040");
    }

    #[test]
    fn distance() {
        let yellow: Rgb<f64> = [250.0, 250.0, 10.0].into();
        let red: Rgb<f64> = [250.0, 10.0, 10.0].into();
        let blue: Rgb<f64> = [10.0, 10.0, 250.0].into();

        let dist_to_red = yellow.distance(&red);
        let dist_to_blue = yellow.distance(&blue);

        assert!(dist_to_red < dist_to_blue)
    }

    #[test]
    fn distance_same_color_0() {
        let a: Rgb<f64> = [220.0, 200.0, 180.0].into();
        let b: Rgb<f64> = [220.0, 200.0, 180.0].into();

        let dist_ab = a.distance(&b);

        assert!(dist_ab < f64::EPSILON)
    }
}
