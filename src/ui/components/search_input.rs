use ratatui::{
    layout::Rect,
    style::Style,
    widgets::Paragraph,
    Frame,
};
use crate::app::App;
use crate::ui::utils::default_block_builder;

pub fn render_search_input(f: &mut Frame, app: &App, area: Rect, dimmed: bool, style: Style) {
    let focus_style = if app.focus == crate::app::AppFocus::Search {
        Style::default().fg(ratatui::style::Color::Yellow)
    } else {
        style
    };

    let block = default_block_builder("Search Input", dimmed).border_style(focus_style);
    
    // Simulate cursor block
    let display_input = if !dimmed && app.focus == crate::app::AppFocus::Search {
        format!("{}_", app.input)
    } else {
        app.input.clone()
    };

    let p = Paragraph::new(display_input).block(block);
    f.render_widget(p, area);
}
