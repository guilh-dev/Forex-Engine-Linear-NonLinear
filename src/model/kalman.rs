pub struct KalmanFilter {
    /// Estimativa do estado atual (ex: preço suavizado)
    pub state: f64,
    /// Incerteza da estimativa (Covariância)
    pub p: f64,
    /// Ruído do processo (Quão rápido o mercado muda intrinsecamente)
    pub q: f64,
    /// Ruído da medição (Volatilidade/Ruído do sensor ou tick)
    pub r: f64,
}

impl KalmanFilter {
    pub fn new(initial_price: f64, q: f64, r: f64) -> Self {
        Self { state: initial_price, p: 1.0, q, r }
    }

    /// Atualiza o filtro com um novo tick e retorna a previsão
    #[inline(always)]
    pub fn update(&mut self, measurement: f64) -> f64 {
        // Equações de Previsão
        let p_pred = self.p + self.q;

        // Ganho de Kalman
        let k = p_pred / (p_pred + self.r);

        // Atualização do Estado
        self.state = self.state + k * (measurement - self.state);
        self.p = (1.0 - k) * p_pred;

        self.state
    }
}