pub struct KalmanFilter {
    /// Estimate of the current state (e.g. smoothed price)
    pub state: f64,
    /// Estimation uncertainty (Covariance)
    pub p: f64,
    /// Market volatility (The speed at which the market changes)
    pub q: f64,
    /// Measurement noise (Volatility/Sensor noise or tick noise)
    pub r: f64,
}

impl KalmanFilter {
    pub fn new(initial_price: f64, q: f64, r: f64) -> Self {
        Self { state: initial_price, p: 1.0, q, r }
    }

    /// Refreshes the filter by ticking a new box and returns the forecast
    #[inline(always)]
    pub fn update(&mut self, measurement: f64) -> f64 {
        let p_pred = self.p + self.q;
        let k = p_pred / (p_pred + self.r);

        self.state = self.state + k * (measurement - self.state);
        self.p = (1.0 - k) * p_pred;

        self.state
    }
}