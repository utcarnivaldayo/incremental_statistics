pub mod incremental_statistics;

#[cfg(test)]
mod tests {
    use crate::incremental_statistics;

    #[test]
    fn test_count() {
        let length = 5;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.count(), 0);

        for i in 1..=length {
            a.add(i as f64);
        }
        assert_eq!(a.count(), length);
    }

    #[test]
    fn test_mean() {
        let length = 5;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.mean(), 0.0f64);

        for i in 1..=length {
            a.add(i as f64);
        }
        // println!("mean: {}", a.mean());
        assert!((a.mean() - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_variance() {
        let length = 5;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.variance(), 0.0f64);

        for i in 1..=length {
            a.add(i as f64);
        }
        // println!("variance: {}", a.variance());
        assert!((a.variance() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_standard_deviation() {
        let length = 5;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.standard_deviation(), 0.0f64);

        for i in 1..=length {
            a.add(i as f64);
        }
        // println!("standard_deviation: {}", a.standard_deviation());
        assert!((a.standard_deviation() - 2.0f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_add_bulk() {
        let arr: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mut a = incremental_statistics::IncrementalStatistics::new();
        a.add_bulk(&arr);
        assert_eq!(a.count(), arr.len());
        assert!((a.mean() - 3.0).abs() < 1e-6);
        assert!((a.standard_deviation() - 2.0f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_clone() {
        let length = 3;
        let mut incres: Vec<incremental_statistics::IncrementalStatistics> =
            vec![incremental_statistics::IncrementalStatistics::new(); length];
        let arr: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        incres[0].add_bulk(&arr);
        incres[1].add_bulk(&arr);
        incres[2].add_bulk(&arr);
        for i in 0..length {
            assert_eq!(incres[i].count(), arr.len());
            assert!((incres[i].mean() - 3.0).abs() < 1e-6);
            assert!((incres[i].standard_deviation() - 2.0f64.sqrt()).abs() < 1e-6);
        }
    }
}
