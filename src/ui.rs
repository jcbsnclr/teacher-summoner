//! This module defines the basic components for the UI that the rest of the application relies upon.

use maud::{Render, DOCTYPE};

/// Creates a link as part of a list
fn list_link(text: &str, url: &str) -> maud::Markup {
    maud::html! {
        li {
            a href=(url) { (text) }
        }
    }
}

/// Renders a sidebar, with a navbar based on a list of `(text, url)`.
pub fn sidebar<'a>(items: impl IntoIterator<Item = (&'a str, &'a str)>) -> maud::Markup {
    maud::html! {
        div class="sidebar" {
            header class="terminal-logo" {
                h1 class="terminal-prompt" {
                    a href="/" { "Teacher Summoner" }
                }
            }
            ul {
                // for each item in navbar, render list item containing a link
                @for item in items {
                    li {
                        a href=(item.1) { (item.0) }
                    }
                }
            }

            // hard-coded project links
            h1 { "Project Links:" }
            ul {
                (list_link("GitHub", "https://github.com/jcbsnclr/teacker-summoner"))
                (list_link("License", "https://www.gnu.org/licenses/agpl-3.0.en.html"))
                li {
                    "Made using "
                    a href="https://www.rust-lang.org" { "Rust" }
                    " and "
                    a href="https://github.com/tokio-rs/axum" { "Axum" }
                }
            }
        }
    }
}

/// The base "template" that all other pages build on top of
pub fn base(title: &str, body: impl Render) -> maud::Markup {
    maud::html! {
        (DOCTYPE)

        head {
            meta charset="utf-8" {}
            title { (title) " - Teacher Summoner" }

            // include stylesheets
            link rel="stylesheet" href="/static/terminal.min.css" {}
            link rel="stylesheet" href="/static/style.css" {}
        }

        body class="terminal" {
            div class="flex-container" {
                (sidebar([
                    ("Create Class", "/create-class"),
                    ("Join Class", "/join-class")
                ]))

                div class="content" {
                    h1 { (title) }
                    (body)
                }
            }
        }
    }
}
