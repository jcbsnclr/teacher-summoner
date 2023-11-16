use maud::{Render, DOCTYPE};

use crate::ticket::Ticket;

const STYLESHEET: &'static str = include_str!("style.css");

pub fn base(title: &str, body: impl Render) -> maud::Markup {
    maud::html! {
        (DOCTYPE)

        head {
            meta charset="utf-8" {}
            title { (title) " - WIP" }
            link rel="stylesheet" href="https://unpkg.com/terminal.css@0.7.2/dist/terminal.min.css" {}
            style { (STYLESHEET) }
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
