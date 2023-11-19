//! Teacher Summoner, a simple web-app to request help from your teacher.
//!
//! Allows users to create a "class", in which other users can open "tickets" to request help.
//! Users can join classes via a 4-digit hexadecimal code. They will be presented with an error if
//! a class does not exist with a given code.
//!
//! The creator of a class is presented with a dynamic view of open tickets. They are able to dismiss
//! tickets as they see to them.

mod class;
mod state;
mod student;
mod teacher;
mod ticket;
mod ui;

use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;

use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // index page for site
        .route("/", get(root))
        // handlers for creating/joining classes
        .route("/create-class", get(class::create))
        .route("/join-class", get(class::join_form))
        .route("/join-class", post(class::join_submit))
        // handler for list of open tickets
        .route("/class/:id/teacher", get(teacher::ticket_list))
        // handlers for entering and submitting tickets
        .route("/class/:id/student", get(student::view))
        .route("/class/:id/student", post(student::submit_ticket))
        // static data (js and stylesheets)
        .nest_service("/static", ServeDir::new("static"))
        // state containing classes and their lists of tickets
        .with_state(state::AppState::init())
        // middleware to insert JS to auto-reload page on request
        // from server
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Index page (path `/`) for application
async fn root() -> maud::Markup {
    ui::base(
        "Welcome!",
        maud::html! {
            p {
                "Welcome to "
                i { "Teacher Summoner" }
                ", a simple web app to request help from the teacher."
            }
            p {
                "Please either "
                a href="/create-class" { "create a class " }
                "or "
                a href="/join-class" { "join a class " }
                "to get started!"
            }
        },
    )
}
