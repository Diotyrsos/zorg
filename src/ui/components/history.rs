use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{List, ListItem},
    Frame,
};
use crate::app::App;
use crate::ui::utils::default_block_builder;

pub fn render_history(f: &mut Frame, app: &App, area: Rect, dimmed: bool, style: Style) {
    let block = default_block_builder("History", dimmed).style(style);

    let mut items: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(format!("LOG: {}", m)))
        .collect();

    let hist_items: Vec<ListItem> = app
        .recent_history
        .iter()
        .map(|h| {
            let conn_name = app
                .connections
                .iter()
                .find(|c| c.id == Some(h.connection_id))
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");
                
            let duration = h.ended_at - h.started_at;
            let status = if h.exit_code == "0" { "OK" } else { "ERR" };
            
            ListItem::new(format!(
                "[{}] {} ({}s) - Exit: {}",
                status, conn_name, duration, h.exit_code
            ))
        })
        .collect();

    items.extend(hist_items);

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
