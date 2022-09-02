mod incremental_statistics;

#[cfg(test)]
mod tests {
    use crate::incremental_statistics;

    #[test]
    fn test_count() {
        let l = 6;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.count(), 0);

        for i in 1..l {
            a.add(i as f64);
        }
        assert_eq!(a.count(), 5);
    }

    #[test]
    fn test_mean() {
        let l = 6;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.mean(), 0.0f64);

        for i in 1..l {
            a.add(i as f64);
        }
        println!("mean: {}", a.mean());
        assert!((a.mean() - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_variance() {
        let l = 6;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.variance(), 0.0f64);

        for i in 1..l {
            a.add(i as f64);
        }
        println!("variance: {}", a.variance());
        assert!((a.variance() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_standard_deviation() {
        let l = 6;
        let mut a = incremental_statistics::IncrementalStatistics::new();
        assert_eq!(a.standard_deviation(), 0.0f64);

        for i in 1..l {
            a.add(i as f64);
        }
        println!("standard_deviation: {}", a.standard_deviation());
        assert!((a.standard_deviation() - 2.0f64.sqrt()).abs() < 1e-6);
    }
}
