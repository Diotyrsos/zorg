use ratatui::{
    layout::Rect,
    widgets::{List, ListItem},
    Frame,
};
use crate::app::App;
use crate::ui::utils::default_block_builder;

pub fn render_search_results(f: &mut Frame, app: &App, area: Rect, dimmed: bool) {
    let block = default_block_builder("Connections / Search Results", dimmed);

    let items: Vec<ListItem> = app
        .connections
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let fav_icon = if c.is_favorite { "★ " } else { "  " };
            let text = format!(
                "{}{} ({}@{}) - Port: {}",
                fav_icon,
                c.name,
                c.username,
                c.hostname,
                c.port.unwrap_or(22)
            );
            
            if i == app.selected_connection_index {
                let bg_color = if app.focus == crate::app::AppFocus::List {
                    ratatui::style::Color::DarkGray
                } else {
                    ratatui::style::Color::Reset
                };
                ListItem::new(text).style(ratatui::style::Style::default().bg(bg_color).fg(ratatui::style::Color::Yellow))
            } else {
                ListItem::new(text)
            }
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
