//! This module contains the interface for creating and joining classes.
//!
//! This module is used to define the following endpoints:
//!   * *GET*  `/create-class` ([`create`])
//!   * *GET*  `/join-class`   ([`join_form`])
//!   * *POST* `/join-class`   ([`join_submit`])

use axum::extract::{Form, State};
use axum::response::Redirect;

use serde::Deserialize;

use crate::state::AppState;
use crate::ui;

/// Create a new clasroom, and redirect user to the teacher's view of the classroom
pub async fn create(State(state): State<AppState>) -> Redirect {
    let id = state.create_class().as_u16();

    Redirect::to(&format!("/class/{id}/teacher"))
}

/// The form presented to the user to join a classroom via a given 4-digit code
pub async fn join_form() -> maud::Markup {
    ui::base(
        "Join Class",
        maud::html! {
            form class="t-form" action="/join-class" method="post" {
                fieldset {
                    legend { "Class" }

                    div class="form-group" {
                        label for="code" { "Class Code: " }
                        input name="code"
                              type="text"
                              placeholder="e.g. 1A3C"
                              // ensure input is a valid hex digit
                              pattern="[0-9a-fA-F]+"
                              // input must be exactly 4 digits
                              minlength="4"
                              maxlength="4"
                              required {}
                    }

                    div class="form-group" {
                        input type="submit" value="Join" class="btn btn-default"  {}
                    }
                }
            }
        },
    )
}

/// Contains the data submitted when the user joins a classroom
#[derive(Deserialize)]
pub struct JoinData {
    /// 4-digit hexadecimal code for the class. Validation handled by HTML form
    code: String,
}

/// Handler for the form submitted via [`join_form`]
#[axum::debug_handler]
pub async fn join_submit(
    State(state): State<AppState>,
    Form(data): Form<JoinData>,
) -> Result<Redirect, maud::Markup> {
    // parse 4-digit hex number into a 16-bit integer
    // this should never fail, as the HTML form enforces valid input, thus `expect` here
    // should never panic
    let code = u16::from_str_radix(&data.code, 16).expect("form produced invalid hex digit");

    state
        // retrieve class of the given code
        .get_code(code)
        // if class exists, redirect to student view
        .map(|_| Redirect::to(&format!("/class/{code}/student")))
        // if class doesn't exist, present an error to user
        .map_err(|e| {
            ui::base(
                "Unknown Class",
                maud::html! {
                    p { (e) "." }
                    a href="/join-class" { "Go back" }
                },
            )
        })
}
