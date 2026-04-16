use super::{kalman::KalmanFilter, rls_online::SgdResidualPredictor};

pub struct HybridForecaster {
    kalman: KalmanFilter,
    ml_residual: SgdResidualPredictor,
}

impl HybridForecaster {
    pub fn new(kalman: KalmanFilter, ml_residual: SgdResidualPredictor) -> Self {
        Self { kalman, ml_residual }
    }

    /// Processes a new tick and returns the forecast for the next tick
    pub fn step(&mut self, current_price: f64, features: &[f64]) -> f64 {
        
        let kalman_estimate = self.kalman.update(current_price);
        let current_residual = current_price - kalman_estimate;
        self.ml_residual.update_weights(features, current_residual);

        let predicted_residual = self.ml_residual.predict_residual(features);

        kalman_estimate + predicted_residual
    }
}