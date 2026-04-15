mod engine;
mod model; // Requer os arquivos da resposta anterior
mod network;

use engine::ring_buffer::FeatureWindow;
use model::kalman::KalmanFilter;
use model::rls_online::SgdResidualPredictor;
use model::hybrid::HybridForecaster;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // 1. Inicializa o motor (mesma lógica anterior)
    let window_size = 30;
    let learning_rate = 0.05;
    let lambda = 0.98; // Fator de Esquecimento
    let num_features = 2; // [Z-Score, Volatilidade]

    // 2. Inicialização dos Componentes
    let mut feature_window = FeatureWindow::new(window_size);
    
    // Assumindo um preço inicial aproximado (ex: EUR/USD)
    let initial_price = 1.0500; 
    let kalman = KalmanFilter::new(initial_price, 1e-5, 1e-3);
    let sgd = SgdResidualPredictor::new(num_features, learning_rate, lambda);
    
    let mut engine = HybridForecaster::new(kalman, sgd);

    // 2. Spawn da task de rede (Thread de IO)
    let url = "wss://stream.binance.com:9443/ws/eurusdt@aggTrade";
    tokio::spawn(async move {
        network::ws_client::start_market_feed(url, tx).await;
    });

    println!("Motor aguardando ticks reais...");

    // 3. O Hot Path (Loop Principal de Execução)
    while let Some(price) = rx.recv().await {
        feature_window.push(price);
        
        if feature_window.is_ready() {
            let features = feature_window.get_features(price);
            let prediction = engine.step(price, &features[..]);
            
            // Aqui você dispararia sua ordem ou log de sinal
            println!("LIVE TICK: {:.5} | PRED (T+1): {:.5}", price, prediction);
        }
    }
}