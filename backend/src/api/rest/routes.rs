use axum::{routing::get, Router};

use super::handlers::{
    get_current_funding, get_funding_history, get_funding_stats, get_next_payment,
    get_payment_history, health_handler,
};

pub fn build_routes() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/funding/current/:symbol", get(get_current_funding))
        .route("/funding/history/:symbol", get(get_funding_history))
        .route("/funding/payments/:position", get(get_payment_history))
        .route("/funding/stats/:symbol", get(get_funding_stats))
        .route("/funding/next-payment", get(get_next_payment))
}

