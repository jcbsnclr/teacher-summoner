mod form;
mod ticket;
mod ui;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::{get, post};
use axum::Form;
use axum::Router;

use form::FormData;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use maud::{Markup, Render};

use ticket::{Ticket, TicketId, TicketList};

#[derive(Clone)]
struct AppState {
    tickets: Arc<Mutex<TicketList>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            tickets: Arc::new(Mutex::new(TicketList::new())),
        }
    }

    pub fn with_tickets<T>(&self, op: impl Fn(&TicketList) -> T) -> T {
        let guard = self.tickets.lock().unwrap();

        op(&guard)
    }

    pub fn with_tickets_mut<T>(&self, op: impl Fn(&mut TicketList) -> T) -> T {
        let mut guard = self.tickets.lock().unwrap();

        op(&mut guard)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/teacher", get(teacher_view))
        .route("/student", get(student_view))
        .route("/student", post(submit_ticket))
        .route("/", get(root))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(AppState::init())
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn submit_ticket(
    State(state): State<AppState>,
    Form(FormData { student, desc }): Form<form::FormData>,
) -> Markup {
    let id = state.with_tickets_mut(|t| t.add_ticket(&student, desc.as_ref()));

    ui::base(
        "Tickets",
        maud::html! {
            div class="terminal-alert terminal-alert-primary" {
                "Success (ticket " (id) " )"
            }
            (form::form())
        },
    )
}

async fn student_view() -> Markup {
    ui::base("Tickets", form::form())
}

async fn teacher_view(State(state): State<AppState>) -> Markup {
    ui::base("Tickets", state.with_tickets(|t| t.render()))
}

async fn root() -> Markup {
    ui::base("Tickets", "WIP")
}
