use super::{kalman::KalmanFilter, rls_online::SgdResidualPredictor};

pub struct HybridForecaster {
    kalman: KalmanFilter,
    ml_residual: SgdResidualPredictor,
}

impl HybridForecaster {
    pub fn new(kalman: KalmanFilter, ml_residual: SgdResidualPredictor) -> Self {
        Self { kalman, ml_residual }
    }

    /// Processa um novo tick e retorna a previsão para o PRÓXIMO tick
    pub fn step(&mut self, current_price: f64, features: &[f64]) -> f64 {
        // 1. O Kalman tenta prever onde estamos (suavização)
        let kalman_estimate = self.kalman.update(current_price);
        
        // 2. Calculamos o erro do Kalman no tick atual
        let current_residual = current_price - kalman_estimate;

        // 3. Treinamos o modelo SGD com o erro que acabou de acontecer
        self.ml_residual.update_weights(features, current_residual);

        // 4. Prevemos o erro FUTURO baseado nas features atuais (ex: Z-Score na janela)
        let predicted_residual = self.ml_residual.predict_residual(features);

        // 5. Sinal Híbrido Final: Previsão Base + Previsão do Erro
        kalman_estimate + predicted_residual
    }
}