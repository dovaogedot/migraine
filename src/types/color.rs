use std::ops::Sub;

use crate::algorithm::kmeans::{Centroid, Distance, Same};

/// Wrapper over RGB color with some utility functions.
///
/// # Examples
///
/// ```
/// let red = Color { r: 255.0, g: 0.0, b: 0.0 };
/// let green = Color { r: 0.0, g: 255.0, b: 0.0 };
/// let blue = Color { r: 0.0, g: 0.0, b: 255.0 };
/// let yellow = Color::new(127.0, 127.0, 0.0);
/// let cyan = Color::new(0.0, 127.0, 127.0);
/// let magenta = Color::new(127.0, 0.0, 127.0);
///
/// assert!(yellow.distance(&red) < yellow.distance(&blue));
/// assert_eq!(yellow.to_rgb_string(), "(127, 127,   0)");
/// assert_eq!(cyan.to_hex_string(), "#007f7f");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn to_rgb_string(&self) -> String {
        format!(
            "({:3}, {:3}, {:3})",
            self.r.round() as u32,
            self.g.round() as u32,
            self.b.round() as u32,
        )
    }

    pub fn to_hex_string(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            self.r.round() as u32,
            self.g.round() as u32,
            self.b.round() as u32,
        )
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.simple_distance(other)
    }

    fn simple_distance(&self, other: &Self) -> f64 {
        let r = self.r - other.r;
        let g = self.g - other.g;
        let b = self.b - other.b;
        ((r * r) * 0.3 + (g * g) * 0.59 + (b * b) * 0.11).sqrt()
    }
}

impl Centroid for Color {
    fn centroid(cluster: &[Self]) -> Self {
        let (r, g, b) = cluster
            .iter()
            .fold((0.0, 0.0, 0.0), |(r, g, b), c| (r + c.r, g + c.g, b + c.b));
        let total = cluster.len() as f64;
        Color::new(r / total, g / total, b / total)
    }
}

impl Distance for Color {
    fn distance(a: &Self, b: &Self) -> f64 {
        a.distance(b)
    }
}

impl Same for Color {
    fn same_clusters(a: &[Self], b: &[Self]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.iter().zip(b).all(|(a, b)| {
            a.r.sub(b.r).abs() < f64::EPSILON
                && a.g.sub(b.g).abs() < f64::EPSILON
                && a.b.sub(b.b).abs() < f64::EPSILON
        })
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.to_hex_string(), self.to_rgb_string())
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Color::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_string() {
        let color = Color::new(255.0, 127.0, 64.0);
        let rgb_string = color.to_rgb_string();
        assert_eq!(rgb_string, "(255, 127,  64)");
    }

    #[test]
    fn hex_string() {
        let color = Color::new(255.0, 127.0, 64.0);
        let hex_string = color.to_hex_string();
        assert_eq!(hex_string, "#ff7f40");
    }

    #[test]
    fn distance() {
        let yellow = Color::new(250.0, 250.0, 10.0);
        let red = Color::new(250.0, 10.0, 10.0);
        let blue = Color::new(10.0, 10.0, 250.0);

        let dist_to_red = yellow.distance(&red);
        let dist_to_blue = yellow.distance(&blue);

        assert!(dist_to_red < dist_to_blue)
    }

    #[test]
    fn distance_same_color_0() {
        let a = Color::new(220.0, 200.0, 180.0);
        let b = Color::new(220.0, 200.0, 180.0);

        let dist_ab = a.distance(&b);

        assert!(dist_ab < f64::EPSILON)
    }
}
