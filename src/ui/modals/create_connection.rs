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
}

impl Default for CreateConnectionModal {
    fn default() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            username: String::new(),
            hostname: String::new(),
            port: String::new(),
            identity_file: String::new(),
            note: String::new(),
            active_field: 0,
        }
    }
}

impl CreateConnectionModal {
    pub fn reset(&mut self) {
        self.name.clear();
        self.username.clear();
        self.hostname.clear();
        self.port.clear();
        self.identity_file.clear();
        self.note.clear();
        self.active_field = 0;
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

        let block = default_block_builder("Create Connection", false);
        f.render_widget(block.clone(), popup_area);

        let inner_area = block.inner(popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Name
                Constraint::Length(3), // Username
                Constraint::Length(3), // Hostname
                Constraint::Length(3), // Port
                Constraint::Length(3), // Identity File
                Constraint::Length(3), // Note
                Constraint::Min(1),    // Button
            ])
            .split(inner_area);

        let fields = vec![
            ("Name", &self.name),
            ("Username", &self.username),
            ("Hostname", &self.hostname),
            ("Port", &self.port),
            ("Identity File", &self.identity_file),
            ("Note", &self.note),
        ];

        for (i, (label, value)) in fields.iter().enumerate() {
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
                format!("{}█", value)
            } else {
                value.to_string()
            };

            let paragraph = Paragraph::new(render_val).block(block);
            f.render_widget(paragraph, chunks[i]);
        }

        let button_style = if self.active_field == 6 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let button = Paragraph::new("Submit (Enter)")
            .alignment(ratatui::layout::Alignment::Center)
            .style(button_style);

        f.render_widget(button, chunks[6]);
    }
}
