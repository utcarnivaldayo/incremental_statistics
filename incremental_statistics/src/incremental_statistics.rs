#[derive(Clone)]
pub struct IncrementalStatistics {
    mean: f64,
    count: usize,
    m_2: f64,
    c_mean: f64,
    c_m_2: f64,
    cached_standard_deviation: f64,
    cached_un_standard_deviation: f64,
    is_cached_standard_deviation: bool,
    is_cached_un_standard_deviation: bool,
}

impl IncrementalStatistics {
    pub fn new() -> IncrementalStatistics {
        IncrementalStatistics {
            mean: 0.0,
            count: 0,
            m_2: 0.0,
            c_mean: 0.0,
            c_m_2: 0.0,
            cached_standard_deviation: 0.0,
            cached_un_standard_deviation: 0.0,
            is_cached_standard_deviation: false,
            is_cached_un_standard_deviation: false,
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

    pub fn standard_deviation(&mut self) -> f64 {
        if self.is_cached_standard_deviation {
            return self.cached_standard_deviation;
        }

        self.cached_standard_deviation = self.variance().sqrt();
        self.is_cached_standard_deviation = true;
        self.cached_standard_deviation
    }

    pub fn un_standard_deviation(&mut self) -> f64 {
        if self.is_cached_un_standard_deviation {
            return self.cached_un_standard_deviation;
        }
        self.cached_un_standard_deviation = self.un_variance().sqrt();
        self.is_cached_un_standard_deviation = true;
        self.cached_un_standard_deviation
    }

    pub fn upper(&mut self) -> f64 {
        self.mean() + self.standard_deviation()
    }

    pub fn lower(&mut self) -> f64 {
        self.mean() - self.standard_deviation()
    }

    pub fn un_upper(&mut self) -> f64 {
        self.mean() + self.un_standard_deviation()
    }

    pub fn un_lower(&mut self) -> f64 {
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
        self.is_cached_standard_deviation = false;
        self.is_cached_un_standard_deviation = false;
    }

    pub fn add_bulk(&mut self, data_bulk: &Vec<f64>) {
        for i in data_bulk {
            self.add(*i);
        }
    }

    pub fn clear(&mut self) {
        self.mean = 0.0;
        self.count = 0;
        self.m_2 = 0.0;
        self.c_mean = 0.0;
        self.c_m_2 = 0.0;
        self.cached_standard_deviation = 0.0;
        self.cached_un_standard_deviation = 0.0;
        self.is_cached_standard_deviation = false;
        self.is_cached_un_standard_deviation = false;
    }

    fn kahan(data: f64, sum: &mut f64, c: &mut f64) {
        let y: f64 = data - *c;
        let t: f64 = *sum + y;
        *c = (t - *sum) - y;
        *sum = t;
    }
}
