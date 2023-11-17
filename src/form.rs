use serde::{Deserialize, Serialize};

use maud::Render;

#[derive(Debug, Clone, Deserialize)]
pub struct FormData {
    pub student: String,
    pub desc: Option<String>,
}

pub fn form() -> maud::Markup {
    maud::html! {
        form class="student-form" action="/student" method="post" {
            fieldset {
                legend { "Ask for Help" }

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
