use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use sqlx::postgres::PgPoolOptions;

use tower_http::cors::{Any, CorsLayer};

use crate::{config::Settings, health::router::HealthCheckRouter, state::AppState};

const MAX_DB_CONNECTIONS: u32 = 10;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn new(settings: Settings) -> anyhow::Result<App> {
        let db_url = settings.db.get_db_url();
        let db_pool = PgPoolOptions::new()
            .max_connections(MAX_DB_CONNECTIONS)
            .connect(&db_url)
            .await
            .context("Could not connect with the database")?;
        let state = Arc::new(AppState { db_pool });

        let router = Router::new()
            .nest("/health", HealthCheckRouter::build())
            .with_state(state.clone())
            .layer(CorsLayer::new().allow_origin(Any));

        Ok(App { router })
    }
}
