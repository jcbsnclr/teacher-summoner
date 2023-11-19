use maud::{Render, DOCTYPE};

pub fn sidebar<'a>(items: impl IntoIterator<Item = (&'a str, &'a str)>) -> maud::Markup {
    maud::html! {
        div class="sidebar" {
            header class="terminal-logo" {
                h1 class="terminal-prompt" { "Teacher Summoner" }
            }
            ul {
                @for item in items {
                    li {
                        a href=(item.1) { (item.0) }
                    }
                }
            }
        }
    }
}

pub fn base(title: &str, body: impl Render) -> maud::Markup {
    maud::html! {
        (DOCTYPE)

        head {
            meta charset="utf-8" {}
            title { (title) " - Teacher Summoner" }
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
