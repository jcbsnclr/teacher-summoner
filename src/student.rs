use axum::extract::{Form, Path, State};

use serde::Deserialize;

use crate::state::AppState;
use crate::ui;

#[derive(Debug, Clone, Deserialize)]
pub struct FormData {
    pub student: String,
    pub desc: String,
}

pub fn form(id: u16) -> maud::Markup {
    let action = format!("/class/{id}/student");

    maud::html! {
        form class="t-form" action=(action) method="post" {
            fieldset {
                legend { "Details" }

                div class="form-group" {
                    label for="student" { "Name: " }
                    input name="student" type="text" required placeholder="John Doe" {}
                }

                div class="form-group" {
                    label for="desc" { "Description: " }
                    input name="desc" type="text" placeholder="A brief description of your problem (optional)" {}
                }

                div class="form-group" {
                    input type="submit" value="Submit" class="btn btn-default" {}
                }
            }
        }
    }
}

pub async fn submit_ticket(
    State(state): State<AppState>,
    Path(class_id): Path<u16>,
    Form(FormData { student, desc }): Form<FormData>,
) -> maud::Markup {
    let Ok(code) = state.get_code(class_id) else {
        return ui::base(
            "Unknown Class",
            maud::html! {
                p { "Unknown class ID " (class_id) "." }
                a href="/join-class" { "Go back." };
            },
        );
    };

    let desc = if desc.trim().is_empty() {
        None
    } else {
        Some(desc)
    };

    let id = state.with_tickets_mut(code, |t| t.add_ticket(&student, desc.as_ref()));

    ui::base(
        "Open Ticket",
        maud::html! {
            div class="terminal-alert terminal-alert-primary" {
                "Success (ticket " (id) " )"
            }
            (form(class_id))
        },
    )
}

pub async fn view(Path(id): Path<u16>) -> maud::Markup {
    ui::base("Ask for Help", form(id))
}
