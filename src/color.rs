use std::ops::Div;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(rgb: (u8, u8, u8)) -> Self {
        Color {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }

    pub fn to_rgb_string(&self) -> String {
        format!("({:3}, {:3}, {:3})", self.r, self.g, self.b)
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x})", self.r, self.g, self.b)
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.simple_distance(other)
    }

    fn simple_distance(&self, other: &Self) -> f64 {
        let rr = self.r as f64 - other.r as f64;
        let gg = self.g as f64 - other.g as f64;
        let bb = self.b as f64 - other.b as f64;
        ((rr * rr) * 0.3 + (gg * gg) * 0.59 + (bb * bb) * 0.11).sqrt()
    }

    pub fn mix(&self, other: &Color) -> Color {
        let r = ((self.r as u16 + other.r as u16) >> 1) as u8;
        let g = ((self.g as u16 + other.g as u16) >> 1) as u8;
        let b = ((self.b as u16 + other.b as u16) >> 1) as u8;
        Color { r, g, b }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.to_hex_string(), self.to_rgb_string())
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color::new(rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix() {
        let a = Color::new((62, 100, 200));
        let b = Color::new((38, 50, 40));
        let mixed = a.mix(&b);
        let expected = Color::new((50, 75, 120));
        assert_eq!(mixed, expected)
    }

    #[test]
    fn mix_min_max() {
        let a = Color::new((0, 255, 0));
        let b = Color::new((255, 0, 255));
        let mixed = a.mix(&b);
        let expected = Color::new((127, 127, 127));
        assert_eq!(mixed, expected)
    }

    #[test]
    fn mix_overflow() {
        let a = Color::new((200, 200, 200));
        let b = Color::new((240, 240, 240));
        let mixed = a.mix(&b);
        let expected = Color::new((220, 220, 220));
        assert_eq!(mixed, expected)
    }

    #[test]
    fn distance() {
        let a = Color::new((220, 200, 180));
        let b = Color::new((240, 220, 200));
        let c = Color::new((140, 120, 100));

        let dist_ab = a.distance(&b);
        let dist_ac = a.distance(&c);

        assert!(dist_ab < dist_ac)
    }

    #[test]
    fn distance_same_color_0() {
        let a = Color::new((220, 200, 180));
        let b = Color::new((220, 200, 180));

        let dist_ab = a.distance(&b);

        assert!(dist_ab < f64::EPSILON)
    }
}
