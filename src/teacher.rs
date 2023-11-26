//! This module defines the endpoints for teachers viewing open tickets.
//!
//! This module is used to defines the endpoint `/class/{id}/teacher`, which accepts two arguments in the
//! query part of the URL:
//!   * `raw: bool` - if `true`, return only the rendered list of tickets, else return skeleton of the UI
//!   * `dismissed: TicketId` - if provided, then dismiss the given ticket

use serde::Deserialize;

use axum::extract::{Path, Query, State};

use maud::Render;

use crate::state::AppState;
use crate::ticket::TicketId;
use crate::ui;

/// The URL query arguments to the teacher view
#[derive(Deserialize)]
pub struct TeacherArgs {
    /// If `Some(id)`, then we dismiss the specified ticket
    dismissed: Option<TicketId>,
    /// If `Some(true)`, then return list of tickets, otherwise just return UI skeleton
    raw: Option<bool>,
}

/// Handler for the teacher's list of tickets. Teacher can dismiss tickets, view will automatically refresh.
pub async fn ticket_list(
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Query(args): Query<TeacherArgs>,
) -> maud::Markup {
    let Ok(code) = state.get_code(id) else {
        // class not found, prompt to create class
        return ui::base(
            "Unknown Class",
            maud::html! {
                p { "Unknown class ID " (id) "." }
                a href="/create-class" { "Create class." };
            },
        );
    };

    if let Some(ticket) = args.dismissed {
        // user has requested to dismiss a ticket
        state.with_tickets_mut(code, |t| t.dismiss(ticket));
    }

    // render the list of tickets to HTML
    let list = state.with_tickets(code, |t| t.render());

    if !args.raw.unwrap_or(false) {
        // present the base UI
        ui::base(
            &format!("Open Tickets (Class {code})"),
            maud::html! {
                btn class="btn btn-primary btn-ghost" onclick="subscribe()" { "Subscribe" }
                hr {}
                // creates div for the ticket list, will be dynamically filled via JS/AJAX
                div id="ticket-list" {}

                // load script to dynamically refresh contents of `#ticket-list`, will refresh on load
                script src="/static/teacher-view.js" classid=(code.as_u16()) {}
            },
        )
    } else {
        // only send the list alone (used to update list dynamically)
        list
    }
}
