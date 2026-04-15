mod engine;
mod model; // Requer os arquivos da resposta anterior

use engine::ring_buffer::FeatureWindow;
use model::kalman::KalmanFilter;
use model::rls_online::SgdResidualPredictor;
use model::hybrid::HybridForecaster;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // 1. Inicializa o motor (mesma lógica anterior)
    let mut feature_window = FeatureWindow::new(30);
    // ... inicialize seu HybridForecaster aqui ...

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