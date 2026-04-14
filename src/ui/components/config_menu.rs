use ratatui::{
    layout::Rect,
    widgets::Paragraph,
    Frame,
};
use crate::ui::utils::default_block_builder;

pub fn render_config_menu(f: &mut Frame, area: Rect, dimmed: bool) {
    let block = default_block_builder("Config Menu", dimmed);
    let p = Paragraph::new("Application config options coming soon...").block(block);
    f.render_widget(p, area);
}
