pub trait Centroid {
    fn centroid(cluster: &[Self]) -> Self
    where
        Self: Sized;
}

pub trait Distance {
    fn distance(a: &Self, b: &Self) -> f64;
}

pub trait Same {
    fn same(a: &Self, b: &Self) -> bool;
}

pub fn kmeans<P>(k: usize, points: &[P]) -> Vec<Vec<P>>
where
    P: Centroid + Distance + Same + Clone,
{
    assert!(
        points.len() >= k,
        "Number of points should not less than number of clusters"
    );

    // Initialize centroids
    let mut centroids: Vec<P> = points.iter().take(k).cloned().collect();
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
        let new_centroids: Vec<P> = clusters.iter().map(|c| P::centroid(c)).collect();

        // Check for convergence
        if new_centroids
            .iter()
            .zip(&centroids)
            .all(|x| P::same(x.0, x.1))
        {
            converged = true;
        } else {
            centroids = new_centroids;
        }
    }

    clusters
}
