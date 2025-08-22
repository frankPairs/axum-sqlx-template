use std::sync::Arc;

use axum::{Router, routing};

use crate::state::AppState;

use super::handlers::health;

pub struct HealthCheckRouter;

impl HealthCheckRouter {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new().route("/", routing::get(health))
    }
}
