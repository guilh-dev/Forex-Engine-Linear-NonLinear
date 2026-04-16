pub struct SgdResidualPredictor {
    /// Linear model weights
    pub weights: Vec<f64>,
    /// learning rate
    pub learning_rate: f64,
    /// Forgetfulness factor (Lambda) to give greater weight to recent data
    pub lambda: f64,
}

impl SgdResidualPredictor {
    pub fn new(num_features: usize, learning_rate: f64, lambda: f64) -> Self {
        Self {
            weights: vec![0.0; num_features],
            learning_rate,
            lambda,
        }
    }

    /// Predict the Kalman filter error
    #[inline(always)]
    pub fn predict_residual(&self, features: &[f64]) -> f64 {
        self.weights.iter().zip(features.iter()).map(|(w, f)| w * f).sum()
    }

    /// Updates the weights based on the actual error observed
    #[inline(always)]
    pub fn update_weights(&mut self, features: &[f64], actual_error: f64) {
        let predicted_error = self.predict_residual(features);
        let gradient = predicted_error - actual_error; // Derivada da perda linear/MSE

        for i in 0..self.weights.len() {
            self.weights[i] = (self.weights[i] * self.lambda) - (self.learning_rate * gradient * features[i]);
        }
    }
}