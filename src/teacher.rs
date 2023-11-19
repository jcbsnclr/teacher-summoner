use serde::Deserialize;

use axum::extract::{Path, Query, State};

use maud::Render;

use crate::state::AppState;
use crate::ticket::TicketId;
use crate::ui;

#[derive(Deserialize)]
pub struct TeacherArgs {
    dismissed: Option<TicketId>,
    raw: Option<bool>,
}

pub async fn ticket_list(
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Query(args): Query<TeacherArgs>,
) -> maud::Markup {
    let Ok(code) = state.get_code(id) else {
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
        log::info!("dismissed ticket {}", ticket);
        state.with_tickets_mut(code, |t| t.dismiss(ticket));
    }

    // render the list of tickets to HTML
    let list = state.with_tickets(code, |t| t.render());

    if !args.raw.unwrap_or(false) {
        // present the entire UI
        ui::base(
            &format!("Open Tickets (Class {code})"),
            maud::html! {
                div id="ticket-list" {
                    (list)
                }

                script src="/static/main.js" classid=(code.as_u16()) {}
            },
        )
    } else {
        // only send the list alone (used to update list dynamically)
        list
    }
}
