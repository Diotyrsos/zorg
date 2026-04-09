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
        .map(|c| {
            ListItem::new(format!(
                "{} ({}@{}) - Port: {}",
                c.name,
                c.username,
                c.hostname,
                c.port.unwrap_or(22)
            ))
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
