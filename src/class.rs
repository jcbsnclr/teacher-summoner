use axum::extract::{Form, State};
use axum::response::{IntoResponse, Redirect, Response};

use serde::Deserialize;

use crate::state::AppState;
use crate::ui;

#[derive(Deserialize)]
pub struct JoinData {
    code: String,
}

pub async fn create(State(state): State<AppState>) -> Redirect {
    let id = state.create_class().as_u16();

    Redirect::to(&format!("/class/{id}/teacher"))
}

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
                              placeholder="1F4B"
                              pattern="[0-9a-fA-F]+"
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

#[axum::debug_handler]
pub async fn join_submit(
    State(state): State<AppState>,
    Form(data): Form<JoinData>,
) -> Result<Redirect, maud::Markup> {
    let code = u16::from_str_radix(&data.code, 16).expect("form produced invalid hex digit");

    state
        .get_code(code)
        .map(|_| Redirect::to(&format!("/class/{code}/student")))
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
