//! This module defines the internal logic for a [`TicketList`]. No endpoints are defined in this module.

use serde::{Deserialize, Serialize};

use maud::Render;

use chrono::{DateTime, Utc};

use std::collections::HashSet;
use std::fmt;

/// List of tickets, and IDs of tickets that have been dismissed
pub struct TicketList {
    /// List of tickets
    tickets: Vec<Ticket>,
    /// A `HashSet` of all the dismissed tickets, for efficient lookup
    dismissed: HashSet<TicketId>,
}

impl TicketList {
    /// Create an empty list
    pub fn new() -> TicketList {
        TicketList {
            tickets: vec![],
            dismissed: HashSet::new(),
        }
    }

    /// Create a new ticket from a student's name and optional description, and return it's ID
    pub fn add_ticket(
        &mut self,
        student: impl AsRef<str>,
        desc: Option<impl AsRef<str>>,
    ) -> TicketId {
        // get `&str`/`Option<&str>` from student/desc
        let student = student.as_ref().trim().to_string();
        let desc = desc.map(|d| d.as_ref().to_string());

        // get the ticket's ID
        let id = TicketId(self.tickets.len());

        // get current time
        let timestamp = Utc::now();

        // push new ticket to list
        self.tickets.push(Ticket {
            id,
            student,
            desc,
            timestamp,
        });

        // return new ID
        id
    }

    /// Dismiss a given ticket
    pub fn dismiss(&mut self, id: TicketId) {
        self.dismissed.insert(id);
    }
}

impl Render for TicketList {
    fn render(&self) -> maud::Markup {
        let tickets = self
            .tickets
            .iter()
            // filter out tickets that have been dismissed
            .filter(|t| !self.dismissed.contains(&t.id));

        // if the length of dismissed == length of ticket list, then all tickets have been dismissed
        let is_empty = self.tickets.len() == self.dismissed.len();

        maud::html! {
            @if is_empty {
                i { "No tickets open" }
            } @ else {
                div class="terminal-timeline" {
                    // render all tickets
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
    /// Name of student opening a ticket
    student: String,
    /// A brief description of the query
    desc: Option<String>,
    /// The timestamp the ticket was created at
    timestamp: DateTime<Utc>,
}

fn print_elapsed(duration: &chrono::Duration) -> String {
    let mins = duration.num_minutes();
    let secs = duration.num_seconds() % 60;

    format!("{mins}:{secs:02} ago")
}

impl Render for Ticket {
    fn render(&self) -> maud::Markup {
        // action to be called when button clicked, dismisses ticket of given ID
        let action = format!("update_list({})", self.id.0);

        maud::html! {
            div class="help-card terminal-card" {
                @let duration = Utc::now() - self.timestamp;

                header { b { (&self.student) } ", " (print_elapsed(&duration)) " [" (self.id)  "]" }

                p class="help-card-text" {
                    b { "Created: " }
                    i { (self.timestamp.format("%c")) }

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
