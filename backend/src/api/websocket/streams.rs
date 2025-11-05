use std::time::Duration;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Query, State},
    response::IntoResponse,
};
use futures::SinkExt;
use serde::Deserialize;
use tokio::sync::broadcast;
use tokio::time::Instant;
use tracing::error;

use crate::services::oracle_manager::OracleManager;

#[derive(Clone)]
pub struct FundingRateBroadcaster {
    tx: broadcast::Sender<String>,
}

impl FundingRateBroadcaster {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub fn broadcast(&self, message: String) {
        let _ = self.tx.send(message);
    }
}

#[derive(Clone)]
pub struct WsState {
    pub broadcaster: FundingRateBroadcaster,
}

#[derive(Debug, Deserialize)]
pub struct WsParams {
    pub symbol: Option<String>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(_state): State<WsState>,
) -> impl IntoResponse {
    let symbol_filter = params.symbol.unwrap_or_else(|| "BTC-PERP".to_string());
    ws.on_upgrade(move |socket| handle_socket(socket, symbol_filter))
}

async fn handle_socket(mut socket: WebSocket, symbol: String) {
    let oracle = OracleManager::new();
    let mut last_rate = 0.0_f64;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut next_payment_epoch = {
        let now = chrono::Utc::now().timestamp();
        (now / 3600 + 1) * 3600
    };

    loop {
        interval.tick().await;
        let now = chrono::Utc::now().timestamp();
        let (mark, index) = tokio::join!(
            oracle.get_mark_price(&symbol),
            oracle.get_index_price(&symbol)
        );
        let (mark, index) = (mark.unwrap_or(0.0), index.unwrap_or(0.0));
        let premium = if index > 0.0 { (mark - index) / index } else { 0.0 };
        let interest = 0.0001 / 86400.0;
        let rate = (premium + interest).clamp(-0.0005, 0.0005);

        // funding_rate every second
        let funding_msg = serde_json::json!({
            "type": "funding_rate",
            "symbol": symbol,
            "funding_rate": rate,
            "premium_index": premium,
            "mark_price": mark,
            "index_price": index,
            "timestamp": now,
        })
        .to_string();
        if socket.send(Message::Text(funding_msg)).await.is_err() {
            break;
        }

        // funding_change on significant delta
        if (rate - last_rate).abs() > 0.00005 {
            let change_msg = serde_json::json!({
                "type": "funding_change",
                "symbol": symbol,
                "prev_rate": last_rate,
                "new_rate": rate,
                "delta": rate - last_rate,
                "timestamp": now,
            })
            .to_string();
            if socket.send(Message::Text(change_msg)).await.is_err() {
                break;
            }
            last_rate = rate;
        }

        // premium/discount alerts
        if premium.abs() > 0.001 {
            let alert_msg = serde_json::json!({
                "type": "alert",
                "symbol": symbol,
                "message": if premium > 0.0 { "High premium detected" } else { "High discount detected" },
                "premium_index": premium,
                "timestamp": now,
            })
            .to_string();
            if socket.send(Message::Text(alert_msg)).await.is_err() {
                break;
            }
        }

        // hourly payment notification
        if now >= next_payment_epoch {
            let pay_msg = serde_json::json!({
                "type": "payment",
                "symbol": symbol,
                "avg_rate": 0.0001,
                "timestamp": now,
            })
            .to_string();
            if socket.send(Message::Text(pay_msg)).await.is_err() {
                break;
            }
            next_payment_epoch += 3600;
        }
    }
}

/// Spawns background tasks that periodically compute and broadcast events.
pub fn spawn_broadcasters(state: WsState, symbol: String) {
    // No-op for now; per-connection producer handles symbol-specific streaming.
    let _ = (state, symbol);
}

