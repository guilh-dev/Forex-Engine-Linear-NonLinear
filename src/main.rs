mod engine;
mod model;
mod network;

use engine::ring_buffer::FeatureWindow;
use model::kalman::KalmanFilter;
use model::rls_online::SgdResidualPredictor;
use model::hybrid::HybridForecaster;
use tokio::sync::mpsc;

fn main() {
    if cfg!(debug_assertions) {
        println!("Debug mode: Using dummy data...");
        simulation();
    } else {
        println!("Release Mode: Executing production code...");
        predict();
    }
}

#[tokio::main]
pub async fn predict() {
    let (tx, mut rx) = mpsc::channel(100);
    
    let window_size = 30;
    let learning_rate = 0.05;
    let lambda = 0.98;
    let num_features = 2;

    let mut feature_window = FeatureWindow::new(window_size);
    
    let initial_price = 1.0500; 
    let kalman = KalmanFilter::new(initial_price, 1e-5, 1e-3);
    let sgd = SgdResidualPredictor::new(num_features, learning_rate, lambda);
    
    let mut engine = HybridForecaster::new(kalman, sgd);

    let url = "wss://stream.binance.com:9443/ws/eurusdt@aggTrade";
    tokio::spawn(async move {
        network::ws_client::start_market_feed(url, tx).await;
    });

    println!("Engine awaiting actual ticks...");

    while let Some(price) = rx.recv().await {
        feature_window.push(price);
        
        if feature_window.is_ready() {
            let features = feature_window.get_features(price);
            let prediction = engine.step(price, &features[..]);
            
            println!("LIVE TICK: {:.5} | PRED (T+1): {:.5}", price, prediction);
        }
    }
}

pub fn simulation() {

    let window_size = 30;
    let learning_rate = 0.05;
    let lambda = 0.98;
    let num_features = 2;

    let mut feature_window = FeatureWindow::new(window_size);
    
    let initial_price = 1.0500; 
    let kalman = KalmanFilter::new(initial_price, 1e-5, 1e-3);
    let sgd = SgdResidualPredictor::new(num_features, learning_rate, lambda);
    
    let mut engine = HybridForecaster::new(kalman, sgd);

    let simulated_ticks = vec![
        1.0501, 1.0503, 1.0502, 1.0505, 1.0504, 1.0508, 1.0507, 1.0510,
        1.0509, 1.0512, 1.0515, 1.0513, 1.0518, 1.0520, 1.0519, 1.0522,
        // Market Opening (e.g. NY Opening – higher volatility and a fall)
        1.0515, 1.0505, 1.0490, 1.0485, 1.0470, 1.0475, 1.0460, 1.0450,
    ];

    for (i, &price) in simulated_ticks.iter().enumerate() {
        feature_window.push(price);
        if !feature_window.is_ready() && i < 5 {
            continue; 
        }

        let features = feature_window.get_features(price);
        let prediction = engine.step(price, &features[..]);

        println!(
            "{:03}  | TICK: {:.5}  | PRED (T+1): {:.5}       | Z-Score: {:>6.2}",
            i, price, prediction, features[0]
        );
    }
    
    println!("Simulation complete. Engine ready for use with WebSocket integration.");
}