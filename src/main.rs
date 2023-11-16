mod ticket;
mod ui;

use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;

use tower_livereload::LiveReloadLayer;

use maud::{Markup, Render};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> Markup {
    ui::base("Tickets", "Hello, World!")
}
