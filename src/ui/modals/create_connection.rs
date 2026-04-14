use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use crate::ui::utils::{center_rect, default_block_builder};

pub struct CreateConnectionModal {
    pub is_open: bool,
    pub name: String,
    pub username: String,
    pub hostname: String,
    pub port: String,
    pub identity_file: String,
    pub note: String,
    pub active_field: usize,
    pub editing_connection_id: Option<i32>,
}

impl Default for CreateConnectionModal {
    fn default() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            username: std::env::var("USER").unwrap_or_default(),
            hostname: String::new(),
            port: String::new(),
            identity_file: String::new(),
            note: String::new(),
            active_field: 0,
            editing_connection_id: None,
        }
    }
}

impl CreateConnectionModal {
    pub fn load_connection(&mut self, conn: &crate::db::connection::Connection) {
        self.editing_connection_id = conn.id;
        self.name = conn.name.clone();
        self.username = conn.username.clone();
        self.hostname = conn.hostname.clone();
        self.port = conn.port.map(|p| p.to_string()).unwrap_or_default();
        self.identity_file = conn.identity_file.clone().unwrap_or_default();
        self.note = conn.note.clone().unwrap_or_default();
        self.active_field = 0;
        self.is_open = true;
    }

    pub fn reset(&mut self) {
        self.name.clear();
        self.username = std::env::var("USER").unwrap_or_default();
        self.hostname.clear();
        self.port.clear();
        self.identity_file.clear();
        self.note.clear();
        self.active_field = 0;
        self.editing_connection_id = None;
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.username.is_empty() && !self.hostname.is_empty()
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        if !self.is_open {
            return;
        }

        let popup_area = center_rect(60, 60, area);
        f.render_widget(Clear, popup_area);

        let title = if self.editing_connection_id.is_some() {
            "Edit Connection"
        } else {
            "Create Connection"
        };
        let block = default_block_builder(title, false);
        f.render_widget(block.clone(), popup_area);

        let inner_area = block.inner(popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Name
                Constraint::Length(3), // Username / Hostname
                Constraint::Length(3), // Port
                Constraint::Length(3), // Identity File
                Constraint::Length(3), // Note
                Constraint::Min(1),    // Button
            ])
            .split(inner_area);

        let user_host_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(chunks[1]);

        let rects = [
            chunks[0],
            user_host_chunks[0],
            user_host_chunks[2],
            chunks[2],
            chunks[3],
            chunks[4],
        ];

        let fields = vec![
            ("Name", &self.name, rects[0]),
            ("Username", &self.username, rects[1]),
            ("Hostname", &self.hostname, rects[2]),
            ("Port", &self.port, rects[3]),
            ("Identity File", &self.identity_file, rects[4]),
            ("Note", &self.note, rects[5]),
        ];

        for (i, (label, value, rect)) in fields.iter().enumerate() {
            let style = if self.active_field == i {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .title(*label)
                .border_style(style);

            let render_val = if self.active_field == i {
                format!("{}_", value)
            } else {
                value.to_string()
            };

            let paragraph = Paragraph::new(render_val).block(block);
            f.render_widget(paragraph, *rect);
        }

        let at_paragraph = Paragraph::new("\n@")
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default());
        f.render_widget(at_paragraph, user_host_chunks[1]);

        let button_style = if self.active_field == 6 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let button = Paragraph::new("[ Submit ]")
            .alignment(ratatui::layout::Alignment::Center)
            .style(button_style);

        f.render_widget(button, chunks[5]);
    }
}
