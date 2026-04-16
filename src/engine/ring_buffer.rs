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

    /// Inserts a new tick without allocating memory (O(1))
    #[inline]
    pub fn push(&mut self, price: f64) {
        self.buffer[self.head] = price;
        self.head += 1;
        if self.head >= self.capacity {
            self.head = 0;
            self.filled = true;
        }
    }

    pub fn is_ready(&self) -> bool {
        self.filled
    }

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
        
        let z_score = if std_dev > 1e-9 { 
            (current_price - mean) / std_dev 
        } else { 
            0.0 
        };

        [z_score, std_dev]
    }
}