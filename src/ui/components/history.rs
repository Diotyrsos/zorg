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

    let items: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
