use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    /// Name of student opening a ticket
    from: String,
    /// A brief description of the query
    desc: Option<String>
}

impl maud::Render for Ticket {
    fn render(&self) -> maud::Markup {
        maud::html! {
            
        }
    }
}