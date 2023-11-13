mod ticket;
mod template;

use std::net::{ToSocketAddrs, SocketAddr};

use axum::Router;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Json;

use maud::{Markup, Render};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127,0,0,1], 1234));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> Markup {
    template::Base {
        title: "Hello, World!".to_string(),
        body: "foo"
    }.render()
}