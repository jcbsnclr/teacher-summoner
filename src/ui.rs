use maud::{Render, DOCTYPE};

use crate::ticket::Ticket;

pub fn base(title: &str, body: impl Render) -> maud::Markup {
    maud::html! {
        (DOCTYPE)

        head {
            meta charset="utf-8" {}
            title { (title) " - WIP" }
            link rel="stylesheet" href="/static/terminal.min.css" {}
            link rel="stylesheet" href="/static/style.css" {}
        }

        body class="terminal" {
            div class="flex-container" {
                div class="sidebar" {
                    header class="terminal-logo" {
                        h1 class="terminal-prompt" { "Teacher Summoner" }
                    }
                    ul {
                        li { "Create Class" }
                        li { "Join Class" }
                    }
                }

                div class="content" {
                    h1 { "Teacher View" }

                    (body)
                }
            }
        }
    }
}
