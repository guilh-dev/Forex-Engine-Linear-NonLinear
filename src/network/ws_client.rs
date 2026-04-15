use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TickData {
    #[serde(rename = "p")] // Exemplo de mapeamento para Binance (p = price)
    pub price: String,
    #[serde(rename = "s")]
    pub symbol: String,
}

pub async fn start_market_feed(url: &str, tx: tokio::sync::mpsc::Sender<f64>) {
    let (ws_stream, _) = connect_async(url).await.expect("Falha ao conectar");
    let (_, mut read) = ws_stream.split();

    println!("Conectado ao Feed: {}", url);

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            // Parsing rápido
            if let Ok(tick) = serde_json::from_str::<TickData>(&text) {
                if let Ok(p) = tick.price.parse::<f64>() {
                    let _ = tx.send(p).await; // Envia o preço para o motor
                }
            }
        }
    }
}