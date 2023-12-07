//! This module defines the endpoints for students accessing a class.
//!
//! This module is used to define the following endpoints:
//!   * *GET*  `/class/{id}/ticket` ([`view`])
//!   * *POST* `/class/{id}/ticket` ([`submit_ticket`])

use axum::extract::{Form, Path, State};

use serde::Deserialize;

use crate::state::AppState;
use crate::ui;

/// Data from ticket details form
#[derive(Debug, Clone, Deserialize)]
pub struct FormData {
    /// The name of the student submitting the ticket
    pub student: String,
    /// An (optional) brief description of the ticket
    pub desc: String,
}

/// The form presented to students to open a ticket. Will `POST` the result to [`submit_ticket`] for
/// a given class.
pub fn form(id: u16) -> maud::Markup {
    // the endpoint to send the form data to
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

/// Handler for any tickets submitted.
pub async fn submit_ticket(
    State(state): State<AppState>,
    Path(class_id): Path<u16>,
    Form(FormData { student, desc }): Form<FormData>,
) -> anyhow::Result<maud::Markup> {
    let Ok(code) = state.get_code(class_id) else {
        // class doesn't exist, present an error and prompt them to join a class
        return Ok(ui::base(
            "Unknown Class",
            maud::html! {
                p { "Unknown class ID " (class_id) "." }
                a href="/join-class" { "Go back." };
            },
        ));
    };

    let desc = if desc.trim().is_empty() {
        // whitespace-trimmed description is empty, produce desc of `None`
        None
    } else {
        Some(desc)
    };

    // add a ticket to the classes' list
    let (sub, id) = state.with_tickets_mut(code, |t| {
        (t.subscriber(), t.add_ticket(&student, desc.as_ref()))
    });

    if let Some(sub) = sub {
        let vapid = state.vapid();
        let msg = sub
            .with_vapid(&vapid, "mailto:jcbsnclr@outlook.com")
            .build("new ticket")?
            .map(|body| body.into());            

        let client = reqwest::Client::new();

        let req = reqwest::RequestBuilder::

        client.execute(msg);
    }

    // present user with message to indicate success
    Ok(ui::base(
        "Open Ticket",
        maud::html! {
            div class="terminal-alert terminal-alert-primary" {
                "Success (ticket " (id) " )"
            }
            (form(class_id))
        },
    ))
}

/// Presents the ticket submission form to the user. Does no additional processing
pub async fn view(Path(id): Path<u16>) -> maud::Markup {
    ui::base("Ask for Help", form(id))
}
