use maud::{Render, DOCTYPE};

const STYLESHEET: &'static str = include_str!("style.css");

pub struct Base<B> {
    pub title: String,
    pub body: B 
}

impl<B> Render for Base<B>
where B: Render {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (DOCTYPE)

            head {
                meta charset="utf-8" {}
                title { (self.title) " - WIP" }
                link rel="stylesheet" href="https://unpkg.com/terminal.css@0.7.2/dist/terminal.min.css" {}
                style { (STYLESHEET) }
            }

            body class="terminal" {
                div class="flex-container" {
                    div class="sidebar" {
                        h1 { "Navigation:" }
                        ul {
                            li { "Foo" }
                            li {" Bar" }
                        }
                    }

                    div class="content" {
                        (self.body)
                    }
                }
            }
        }
    }
}