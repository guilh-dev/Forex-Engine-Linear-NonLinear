pub struct FeatureWindow {
    buffer: Vec<f64>,
    capacity: usize,
    head: usize,
    filled: bool,
}

impl FeatureWindow {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0.0; capacity],
            capacity,
            head: 0,
            filled: false,
        }
    }

    /// Insere um novo tick sem alocar memória (O(1))
    #[inline]
    pub fn push(&mut self, price: f64) {
        self.buffer[self.head] = price;
        self.head += 1;
        if self.head >= self.capacity {
            self.head = 0;
            self.filled = true;
        }
    }

    /// Retorna se a janela já tem dados suficientes para operar
    pub fn is_ready(&self) -> bool {
        self.filled
    }

    /// Calcula a Média e o Desvio Padrão (Volatilidade Realizada)
    pub fn stats(&self) -> (f64, f64) {
        let count = if self.filled { self.capacity } else { self.head };
        if count == 0 { return (0.0, 0.0); }

        let sum: f64 = self.buffer[..count].iter().sum();
        let mean = sum / count as f64;

        let variance: f64 = self.buffer[..count]
            .iter()
            .map(|&value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f64>() / count as f64;

        (mean, variance.sqrt())
    }

    /// Calcula os Features para o SGD: [Z-Score, Volatilidade]
    pub fn get_features(&self, current_price: f64) -> [f64; 2] {
        let (mean, std_dev) = self.stats();
        
        // Proteção contra divisão por zero
        let z_score = if std_dev > 1e-9 { 
            (current_price - mean) / std_dev 
        } else { 
            0.0 
        };

        // Agora ambos são f64, prontos para o SGD ler como features[0] e features[1]
        [z_score, std_dev]
    }
}