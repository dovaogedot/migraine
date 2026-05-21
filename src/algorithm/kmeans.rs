/// Centroid of a cluster.
pub trait Centroid: Sized {
    fn centroid(cluster: &[Self]) -> Self;
}

/// Distance metric between two points in a set.
pub trait Distance: Sized {
    fn distance(a: &Self, b: &Self) -> f64;
}

/// Metric used to decide whether two clusters are converged.
pub trait Same: Sized {
    fn same_clusters(a: &[Self], b: &[Self]) -> bool;
}

/// Implementation of naive k-means algorithm.
///
/// Ripped directly ripped from its [Wikipedia article](https://en.wikipedia.org/wiki/K-means_clustering#Standard_algorithm_(naive_k-means)).
///
/// # Examples
///
/// ```
/// struct Point(u32);
///
/// impl Distance for Point {
///     fn distance(a: &Self, b: &Self) -> f64 {
///         (a.0 - b.0) as f64
///     }
/// }
///
/// impl Same for Point {
///     fn same(a: &Self, b: &Self) -> bool {
///         a.0 == b.0
///     }
/// }
///
/// impl Centroid for Point {
///     fn centroid(cluster: &[Self]) -> Self {
///         cluster.iter().sum() / cluster.len()
///     }
/// }
///
/// let k = 2;
/// let points = vec![Point(3), Point(1), Point(9), Point(11), Point(8)];
/// let initial_clusters = points.iter().take(4).cloned().collect();
/// let clusters = kmeans(k, points, initial_clusters);
/// assert_eq!(clusters.len(), 2);
/// ```
pub fn kmeans<P>(k: usize, points: &[P], initial_centroids: Vec<P>) -> Vec<Vec<P>>
where
    P: Centroid + Distance + Same + Clone,
{
    assert!(k > 0, "Number of cluster must be greater than zero");
    assert!(
        points.len() >= k,
        "Number of points should not be less than number of clusters"
    );

    // Initialize centroids
    let mut centroids = initial_centroids;
    let mut converged = false;
    let mut clusters: Vec<Vec<P>> = Vec::with_capacity(k);

    while !converged {
        // Create empty clusters
        clusters = vec![vec![]; k];

        // Assign each point to the nearest centroid
        for point in points {
            let mut closest_index = 0;
            let mut min_distance = P::distance(point, &centroids[0]);
            for j in 1..k {
                let d = P::distance(point, &centroids[j]);
                if d < min_distance {
                    min_distance = d;
                    closest_index = j;
                }
            }
            clusters[closest_index].push(point.clone());
        }

        // Recalculate centroids as the mean of each cluster
        let mut new_centroids = Vec::with_capacity(k);
        for i in 0..k {
            if clusters[i].is_empty() {
                new_centroids.push(centroids[i].clone());
            } else {
                new_centroids.push(P::centroid(&clusters[i]))
            }
        }

        // Check for convergence
        if P::same_clusters(&new_centroids, &centroids)
        {
            converged = true;
        } else {
            centroids = new_centroids;
        }
    }

    clusters
}
