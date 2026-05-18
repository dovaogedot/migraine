use std::marker::PhantomData;

/// Pattern for sub-pixel sampling.
pub struct SamplePattern {
    /// Array of positions within the pixel.
    ///
    /// ```
    ///  (-1, 1) +---------+ (1, 1)
    ///          |    |    |
    ///          |--(0,0)--|
    ///          |    |    |
    /// (-1, -1) +---------+ (1, -1)
    /// ```
    pub positions: Vec<(f64, f64)>,
    _private: PhantomData<bool>,
}

impl Default for SamplePattern {
    fn default() -> Self {
        SamplePattern::center().combine(&SamplePattern::grid())
    }
}

impl SamplePattern {
    pub fn new(positions: Vec<(f64, f64)>) -> Self {
        assert!(
            positions.len() > 0,
            "Sampling pattern must have at least one position"
        );
        SamplePattern {
            positions,
            _private: PhantomData,
        }
    }

    pub fn combine(&self, other: &Self) -> Self {
        let mut positions = self.positions.clone();
        positions.extend(&other.positions);
        SamplePattern::new(positions)
    }

    pub fn center() -> Self {
        SamplePattern::new(vec![(0.0, 0.0)])
    }

    pub fn grid() -> Self {
        let d = 0.5;
        let positions = vec![(-d, d), (d, d), (-d, -d), (d, -d)];
        SamplePattern::new(positions)
    }
}
