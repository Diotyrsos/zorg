use ratatui::{
    layout::Rect,
    widgets::Paragraph,
    Frame,
};
use crate::ui::utils::default_block_builder;

pub fn render_history(f: &mut Frame, area: Rect, dimmed: bool) {
    let block = default_block_builder("History", dimmed);
    let p = Paragraph::new("History coming soon...").block(block);
    f.render_widget(p, area);
}
