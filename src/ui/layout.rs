use crate::app::App;
use crate::ui::components::{
    config_menu::render_config_menu, history::render_history, search_input::render_search_input,
    search_results::render_search_results,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let dimmed = app.create_connection_modal.is_open;
    let style = if dimmed {
        Style::default().fg(Color::Indexed(244))
    } else {
        Style::default()
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_search_input(f, app, left_chunks[0], dimmed, style);
    render_search_results(f, app, left_chunks[1], dimmed);
    render_history(f, app, right_chunks[0], dimmed, style);
    render_config_menu(f, right_chunks[1], dimmed);
}
