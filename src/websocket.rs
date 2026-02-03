//! WebSocket connection to Kraken for real-time price updates.

use crate::config::Config;
use crate::ticker::TickerState;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const KRAKEN_WS: &str = "wss://ws.kraken.com/v2";

#[derive(Serialize)]
struct SubscribeMessage {
    method: String,
    params: SubscribeParams,
}

#[derive(Serialize)]
struct SubscribeParams {
    channel: String,
    symbol: Vec<String>,
    snapshot: bool,
}

#[derive(Deserialize)]
struct WsMessage {
    channel: Option<String>,
    data: Option<Vec<TickerData>>,
}

#[derive(Deserialize)]
struct TickerData {
    symbol: Option<String>,
    last: Option<f64>,
    change: Option<f64>,
}

/// Main WebSocket loop with automatic reconnection.
#[tokio::main]
pub async fn run(state: &Arc<Mutex<TickerState>>, config: &Config) {
    let symbols: Vec<String> = config.coins.iter()
        .map(|c| c.symbol.clone())
        .collect();

    loop {
        if let Err(e) = connect_and_stream(state, &symbols).await {
            eprintln!("WebSocket error: {:?}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn connect_and_stream(
    state: &Arc<Mutex<TickerState>>,
    symbols: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (ws_stream, _) = connect_async(KRAKEN_WS).await?;
    let (mut write, mut read) = ws_stream.split();

    let subscribe = SubscribeMessage {
        method: "subscribe".to_string(),
        params: SubscribeParams {
            channel: "ticker".to_string(),
            symbol: symbols.to_vec(),
            snapshot: true,
        },
    };

    write.send(Message::Text(serde_json::to_string(&subscribe)?)).await?;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    if ws_msg.channel.as_deref() == Some("ticker") {
                        if let Some(data) = ws_msg.data {
                            if let Ok(mut state) = state.lock() {
                                for ticker in data {
                                    if let (Some(symbol), Some(price)) = (ticker.symbol, ticker.last) {
                                        state.update_price(&symbol, price);
                                        if let Some(change) = ticker.change {
                                            let open = price - change;
                                            if open > 0.0 {
                                                state.set_open_price(&symbol, open);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = write.send(Message::Pong(data)).await;
            }
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
    }

    Ok(())
}
