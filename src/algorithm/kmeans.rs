//! Implementation of naive k-means clustering algorithms.
//!
//! This module provides standard k-means clustering, k-means++ initialization,
//! and automated cluster evaluation ($k$-determination) via the elbow method.
//!
//! # Examples
//!
//! All points in the dataset must implement the following traits: [`Centroid`], [`Distance`], and [`Same`].
//!
//! ```rust
//! use migraine::algorithm::kmeans::{Distance, Same, Centroid, kmeans, kmeans_pp, kmeans_elbow};
//!
//! #[derive(Copy, Clone, Debug)]
//! struct Float(f64);
//!
//! impl From<f64> for Float {
//!     fn from(value: f64) -> Self {
//!         Float(value)
//!     }
//! }
//!
//! impl Distance for Float {
//!     fn distance(&self, b: &Self) -> f64 {
//!         (self.0 - b.0).abs()
//!     }
//! }
//!
//! impl Same for Float {
//!     fn same(&self, b: &Self) -> bool {
//!         (self.0 - b.0).abs() < f64::EPSILON
//!     }
//! }
//!
//! impl Centroid for Float {
//!     fn centroid(cluster: &[&Self]) -> Self {
//!         if cluster.is_empty() { return Float(0.0); }
//!         Float(cluster.iter().map(|x| x.0).sum::<f64>() / cluster.len() as f64)
//!     }
//! }
//!
//! let points = vec![Float(1.0), Float(2.0), Float(10.0), Float(11.0)];
//!
//! // If number of clusters and their approximate locations are known in advance, use `kmeans`:
//! let initial_centroids = vec![Float(0.0), Float(8.0)];
//! let clusters = kmeans(2, &points, initial_centroids);
//! assert_eq!(clusters.len(), 2);
//!
//! // If only the number of clusters is known, use `kmeans_pp`:
//! let clusters_pp = kmeans_pp(2, &points);
//! assert_eq!(clusters_pp.len(), 2);
//!
//! // Otherwise, use `kmeans_elbow` to automatically discover the best configuration:
//! let clusters_elbow = kmeans_elbow(&points, Some(3));
//! ```

use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// A trait for types capable of computing the geometric center (centroid) of a group of items.
pub trait Centroid: Sized {
    /// Computes the centroid given a slice of references to items within a single cluster.
    fn centroid(cluster: &[&Self]) -> Self;
}

/// A trait for computing the distance metric between two points in a space.
pub trait Distance: Sized {
    /// Computes the distance between `self` and `other`.
    fn distance(&self, other: &Self) -> f64;
}

/// A metric used to evaluate convergence by determining if two entities are effectively equivalent.
pub trait Same: Sized {
    /// Returns `true` if `self` and `other` have stabilized and are considered the same.
    fn same(&self, other: &Self) -> bool;
}

/// A cluster containing a collection of references to points and a lazily cached centroid.
#[derive(Clone)]
pub struct Cluster<'a, P> {
    points: Vec<&'a P>,
    centroid: Option<P>,
}

impl<'a, P: Clone + Centroid> Cluster<'a, P> {
    /// Clears all assigned points from the cluster and invalidates the cached centroid.
    fn clear(&mut self) {
        self.points.clear();
        self.centroid = None;
    }

    /// Adds a point reference to the cluster and invalidates the cached centroid.
    fn add(&mut self, point: &'a P) {
        self.points.push(point);
        self.centroid = None;
    }

    /// Returns `true` if the cluster contains no points.
    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the centroid of the cluster.
    ///
    /// If the centroid has already been calculated, the cached value is returned.
    /// Otherwise, it recomputes, caches, and returns it.
    pub fn centroid(&mut self) -> P {
        match &self.centroid {
            Some(c) => c.clone(),
            None => {
                let c = P::centroid(&self.points);
                self.centroid = Some(c.clone());
                c
            }
        }
    }

    /// Returns a slice of shared references to the points belonging to this cluster.
    pub fn points(&self) -> &[&P] {
        &self.points
    }
}

impl<'a, P> Default for Cluster<'a, P> {
    fn default() -> Self {
        Self {
            points: Default::default(),
            centroid: Default::default(),
        }
    }
}

/// Performs standard (naive) k-means clustering using user-specified initial centroids.
///
/// # Panics
///
/// - Panics if `k` is 0.
/// - Panics if the number of points is less than `k`.
/// - Panics if the number of initial centroids does not equal `k`.
pub fn kmeans<P>(k: usize, points: &[P], initial_centroids: Vec<P>) -> Vec<Cluster<'_, P>>
where
    P: Centroid + Distance + Same + Clone,
{
    assert!(k > 0, "Number of cluster must be greater than zero");
    assert!(
        points.len() >= k,
        "Number of points should not be less than number of clusters"
    );
    assert!(
        initial_centroids.len() == k,
        "Number of initial centroids must be equal to number of clusters"
    );

    // Initialize centroids and clusters arrays
    let mut centroids: Vec<P> = initial_centroids;
    let mut clusters: Vec<Cluster<P>> = vec![Cluster::default(); k];

    // Will stop when recalculating centroids if they didn't change since last iteration
    let mut converged = false;

    while !converged {
        // Clear clusters
        clusters.iter_mut().for_each(|c| c.clear());

        // Assign each point to the nearest centroid
        for point in points {
            // Find closest centroid
            let mut closest_index = 0;
            let mut min_distance = point.distance(&centroids[0]);

            for i in 1..k {
                let d = point.distance(&centroids[i]);
                if d < min_distance {
                    min_distance = d;
                    closest_index = i;
                }
            }

            // Add point to corresponding cluster
            clusters[closest_index].add(point);
        }

        let mut changed = false;
        // Update centroids to the mean of each cluster
        for i in 0..k {
            if !clusters[i].is_empty() {
                let old = &centroids[i];
                let new = clusters[i].centroid();
                if !old.same(&new) {
                    changed = true
                }
                centroids[i] = new;
            }
        }

        converged = !changed
    }

    clusters
}

/// Performs k-means++ clustering, automatically initializing smart seed centroids.
///
/// This variation initializes centroids sequentially, choosing subsequent points with a probability
/// proportional to their squared distance (`D(x)^2`) from the nearest existing centroid. This significantly
/// speeds up convergence and yields lower overall error variance compared to naive random initialization.
pub fn kmeans_pp<P>(k: usize, points: &[P]) -> Vec<Cluster<'_, P>>
where
    P: Centroid + Distance + Same + Clone,
{
    // Initialize list of centroids with one randomly selected point
    let mut centroids: Vec<P> = Vec::with_capacity(k);
    let first_index = fastrand::usize(0..points.len());
    centroids.push(points[first_index].clone());

    let mut distances_squared: Vec<f64> = vec![0.0; points.len()];

    // Compute squared distance to first centroid
    for (i, point) in points.iter().enumerate() {
        distances_squared[i] = point.distance(&centroids[0]).powi(2);
    }

    // Choose remaining k - 1 centroids
    while centroids.len() < k {
        // Choose next centroid with probability proportional to D(x)^2.
        //
        // The farther the point is from any centroid, the higher probability
        // of it becoming a new centroid.
        let total: f64 = distances_squared.iter().sum();
        let threshold = fastrand::f64() * total;
        let mut cumulative = 0.0;

        for i in 0..points.len() {
            cumulative += distances_squared[i];
            if cumulative >= threshold {
                centroids.push(points[i].clone());
                break;
            }
        }

        let new_centroid = &centroids[centroids.len() - 1];

        // Update minimum distances if new centroid is closer
        for (i, point) in points.iter().enumerate() {
            let d = point.distance(&new_centroid).powi(2);
            distances_squared[i] = distances_squared[i].min(d);
        }
    }

    kmeans(k, points, centroids)
}

/// Automatically finds the optimal number of clusters (`k`) using the elbow method.
///
/// This function evaluates values of `k` from 2 up to `max_k` (or the length of `points`, whichever is lower)
/// in parallel using `rayon`. It determines the ideal "elbow point" by looking at the maximum point of curvature
/// (calculated through the second derivative of the normalized Sum of Squared Errors (SSE)) and returns the optimal clusters.
///
/// # Arguments
///
/// * `points` - A parallel-compatible slice of dataset points.
/// * `max_k` - An optional upper bound for `k`. If `None`, defaults to the total number of items in `points`.
pub fn kmeans_elbow<P: Centroid + Distance + Same + Clone + Sync + Send>(
    points: &[P],
    max_k: Option<u32>,
) -> Vec<Cluster<'_, P>> {
    let min_k = 2;
    let max_k = match max_k {
        None => points.len(),
        Some(k) => points.len().min(k as usize),
    };

    let range_len = max_k - min_k + 1;

    let results: Vec<(f64, Vec<Cluster<P>>)> = (min_k..=max_k)
        .into_par_iter()
        .map(|k| {
            let mut clusters = kmeans_pp(k, points);

            let mut local_sse = 0.0;
            for cluster in clusters.iter_mut() {
                let cluster_mean = cluster.centroid();
                for &point in cluster.points() {
                    local_sse += point.distance(&cluster_mean).powi(2);
                }
            }

            (local_sse, clusters)
        })
        .collect();

    let (mut sse, mut clusters_cache): (Vec<f64>, Vec<Vec<Cluster<P>>>) = results.into_iter().unzip();

    // Normalize sums of squared errors
    let sse_max = sse[0];
    for i in 0..sse.len() {
        sse[i] = sse[i] / sse_max;
    }

    let width = 120.0;
    let sse_str = sse
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let w = width * x;
            format!("{:2} {:.4} {}", i + min_k, x, "=".repeat(w as usize))
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("SSE:\n{}", sse_str);

    let mut best_k_idx = 0;
    let mut min_curvature = f64::MAX;

    // Find the elbow using second derivative
    for i in 1..(range_len - 1) {
        let dyl = sse[i] - sse[i - 1];
        let dyr = sse[i + 1] - sse[i];
        let diff = dyl - dyr;

        if diff < min_curvature {
            min_curvature = diff;
            best_k_idx = i;
        }
    }

    clusters_cache.swap_remove(best_k_idx + 1)
}
