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
        .route("/class/:id/teacher", get(teacher::ticket_list))
        .route("/class/:id/student", get(student::view))
        .route("/class/:id/student", post(student::submit_ticket))
        //        .route("/student", get(student::view))
        //        .route("/student", post(student::submit_ticket))
        .route("/create-class", get(class::create))
        .route("/join-class", get(class::join_form))
        .route("/join-class", post(class::join_submit))
        // .route("/class/:class_id/student", get(student::view))
        .route("/", get(root))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state::AppState::init())
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> maud::Markup {
    ui::base(
        "Welcome!",
        maud::html! {
            p { "Welcome to " i { "Teacher Summoner" } ", a simple web app to request help from the teacher!"}
            p { "Please either " a href="/create-class" { "create a class " } "or " a href="/join-class" { "join a class " } "to get started!" }
        },
    )
}
