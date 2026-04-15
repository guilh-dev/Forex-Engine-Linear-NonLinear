pub struct SgdResidualPredictor {
    /// Pesos do modelo linear (ex: [peso_zscore, peso_volatilidade])
    pub weights: Vec<f64>,
    /// Taxa de aprendizado
    pub learning_rate: f64,
    /// Fator de esquecimento (Lambda) para dar mais peso a dados recentes
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

    /// Prevê qual será o erro do Filtro de Kalman
    #[inline(always)]
    pub fn predict_residual(&self, features: &[f64]) -> f64 {
        self.weights.iter().zip(features.iter()).map(|(w, f)| w * f).sum()
    }

    /// Atualiza os pesos baseado no erro real observado
    #[inline(always)]
    pub fn update_weights(&mut self, features: &[f64], actual_error: f64) {
        let predicted_error = self.predict_residual(features);
        let gradient = predicted_error - actual_error; // Derivada da perda linear/MSE

        for i in 0..self.weights.len() {
            // Aplica o Fator de Esquecimento (Lambda) decaindo os pesos antigos
            // e atualiza com o novo gradiente
            self.weights[i] = (self.weights[i] * self.lambda) - (self.learning_rate * gradient * features[i]);
        }
    }
}