mod app;
mod config;
mod health;
mod state;
mod telemetry;

use anyhow::Context;
use app::App;

use config::get_app_settings;

use telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(String::from("axum-sqlx-template"), String::from("info"));

    // Initialize tracing subscriber
    init_subscriber(subscriber);

    let settings = get_app_settings().expect("Unable to get server settings");
    let app = App::new(settings.clone()).await?;

    let addr = settings.application.get_addr()?;
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Could not bind a connection with the server.")?;

    tracing::info!("Server running on {}", addr);

    axum::serve(listener, app.router)
        .await
        .context("Could not run the server.")?;

    Ok(())
}
