pub struct IncrementalStatistics {
    mean: f64,
    count: usize,
    m_2: f64,
    c_mean: f64,
    c_m_2: f64,
}

impl IncrementalStatistics {
    pub fn new() -> IncrementalStatistics {
        IncrementalStatistics {
            mean: 0.0,
            count: 0,
            m_2: 0.0,
            c_mean: 0.0,
            c_m_2: 0.0,
        }
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn variance(&self) -> f64 {
        if self.count != 0usize {
            self.m_2 / self.count as f64
        } else {
            0.0f64
        }
    }

    pub fn un_variance(&self) -> f64 {
        if self.count > 1usize {
            self.m_2 / (self.count - 1) as f64
        } else {
            0.0f64
        }
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn un_standard_deviation(&self) -> f64 {
        self.un_variance().sqrt()
    }

    pub fn upper(&self) -> f64 {
        self.mean() + self.standard_deviation()
    }

    pub fn lower(&self) -> f64 {
        self.mean() - self.standard_deviation()
    }

    pub fn un_upper(&self) -> f64 {
        self.mean() + self.un_standard_deviation()
    }

    pub fn un_lower(&self) -> f64 {
        self.mean() - self.un_standard_deviation()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn add(&mut self, data: f64) {
        self.count += 1;
        let delta: f64 = data - self.mean;
        IncrementalStatistics::kahan(delta / self.count as f64, &mut self.mean, &mut self.c_mean);
        let delta_2: f64 = data - self.mean;
        IncrementalStatistics::kahan(delta * delta_2, &mut self.m_2, &mut self.c_m_2);
    }

    fn kahan(data: f64, sum: &mut f64, c: &mut f64) {
        let y: f64 = data - *c;
        let t: f64 = *sum + y;
        *c = (t - *sum) - y;
        *sum = t;
    }
}
