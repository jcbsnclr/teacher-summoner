use serde::{Deserialize, Serialize};

use maud::Render;

use std::collections::HashSet;
use std::fmt;

use crate::state::ClassCode;

pub struct TicketList {
    class: ClassCode,
    tickets: Vec<Ticket>,
    dismissed: HashSet<TicketId>,
}

impl TicketList {
    pub fn new(class: ClassCode) -> TicketList {
        TicketList {
            class,
            tickets: vec![],
            dismissed: HashSet::new(),
        }
    }

    pub fn add_ticket(
        &mut self,
        student: impl AsRef<str>,
        desc: Option<impl AsRef<str>>,
    ) -> TicketId {
        let student = student.as_ref().to_string();
        let desc = desc.map(|d| d.as_ref().to_string());

        let id = TicketId(self.tickets.len());

        self.tickets.push(Ticket {
            class: self.class,
            id,
            student,
            desc,
        });

        id
    }

    pub fn dismiss(&mut self, id: TicketId) {
        self.dismissed.insert(id);
    }
}

impl Render for TicketList {
    fn render(&self) -> maud::Markup {
        let tickets = self
            .tickets
            .iter()
            .filter(|t| !self.dismissed.contains(&t.id));

        let is_empty = self.tickets.len() == self.dismissed.len();

        maud::html! {
            @if is_empty {
                i { "No tickets open" }
            } @ else {
                div class="terminal-timeline" {
                    @for ticket in tickets {
                        (ticket)
                    }
                }
            }
        }
    }
}

/// Ticket ID. Newtype ensures anywhere we ask for a `TicketId`, it is valid
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketId(usize);

impl fmt::Display for TicketId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "#{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    /// ID of the ticket
    id: TicketId,
    /// Class ID of ticket
    class: ClassCode,
    /// Name of student opening a ticket
    student: String,
    /// A brief description of the query
    desc: Option<String>,
}

impl Render for Ticket {
    fn render(&self) -> maud::Markup {
        let action = format!("update_list({})", self.id.0);

        maud::html! {
            div class="help-card terminal-card" {
                header { (&self.student) " [" (self.id)  "]" }

                p class="help-card-text" {
                    b { "Description: " }
                    @if let Some(desc) = &self.desc {
                        (desc)
                    } @else {
                        i { "No description provided" }
                    }
                }

                button class="btn help-card-btn" onclick=(action) { "[x]" }
            }
        }
    }
}
