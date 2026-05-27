/// A single point in a sub-sampling pattern.
///
/// ```
///  (-1, 1) +---------+ (1, 1)
///          |    |    |
///          |--(0,0)--|
///          |    |    |
/// (-1, -1) +---------+ (1, -1)
/// ```
#[derive(Clone, Debug)]
pub struct SamplePoint {
    /// x-offset in range [-1, 1].
    pub dx: f64,
    /// y-offset in range [-1, 1].
    pub dy: f64,
    /// How much this sample contributes to final color.
    pub weight: f64,
}

impl SamplePoint {
    pub fn new(dx: f64, dy: f64, weight: f64) -> Self {
        SamplePoint { dx, dy, weight }
    }
}

impl From<&(f64, f64, f64)> for SamplePoint {
    fn from(value: &(f64, f64, f64)) -> Self {
        SamplePoint::new(value.0, value.1, value.2)
    }
}

/// Pattern for sub-pixel sampling.
pub struct SamplePattern {
    /// Array of sampling points within the pixel.
    points: Vec<SamplePoint>,
}

impl Default for SamplePattern {
    fn default() -> Self {
        SamplePattern::weighted_center()
    }
}

impl SamplePattern {
    pub fn new(points: Vec<SamplePoint>) -> Self {
        assert!(points.len() > 0, "Sampling pattern must have at least one position");
        SamplePattern { points }
    }

    pub fn points(&self) -> &[SamplePoint] {
        &self.points
    }

    #[allow(unused)]
    pub fn weighted_center() -> Self {
        let d = 0.5;
        let points = [(-d, d, 1.0), (d, d, 1.0), (-d, -d, 1.0), (d, -d, 1.0), (0.0, 0.0, 10.0)];
        SamplePattern::from(points.as_slice())
    }

    #[allow(unused)]
    pub fn center() -> Self {
        SamplePattern::new(vec![SamplePoint::new(0.0, 0.0, 1.0)])
    }

    #[allow(unused)]
    pub fn grid() -> Self {
        let d = 0.5;
        let points = [(-d, d, 0.25), (d, d, 0.25), (-d, -d, 0.25), (d, -d, 0.25)];
        SamplePattern::from(points.as_slice())
    }
}

impl From<&[(f64, f64, f64)]> for SamplePattern {
    fn from(value: &[(f64, f64, f64)]) -> Self {
        SamplePattern::new(value.iter().map(SamplePoint::from).collect())
    }
}
