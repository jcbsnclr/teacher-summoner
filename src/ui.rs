use maud::{Render, DOCTYPE};

const STYLESHEET: &'static str = include_str!("style.css");

fn ticket(student: &str, desc: Option<&str>) -> maud::Markup {
    maud::html! {
        div class="terminal-card" style="margin: 1em; max-width: 60em" {
            header { (student) }

            div style="padding: 1em; display: flex; flex-direction: column;" {
                @if let Some(desc) = desc {
                    b { "Description:" }
                    p { (desc) }
                } @else {
                    p { "No description provided" }
                }

                button class="btn" style="max-width: 5em; align-self: flex-end;" { "[x]" }
            }
        }
    }
}

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

                    div class="terminal-timeline" {
                        (ticket("John Doe", Some("my shit doesn't work help")))
                        (ticket("Jane Doe", None))
                    }
                }
            }
        }
    }
}
