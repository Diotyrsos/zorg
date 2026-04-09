use crate::db::connection::{Connection, NewConnection};
use crate::ui::modals::create_connection::CreateConnectionModal;
use diesel::SqliteConnection;

pub struct App {
    pub input: String,
    pub cursor_position: usize,
    pub messages: Vec<String>,
    pub create_connection_modal: CreateConnectionModal,
    pub db: SqliteConnection,
    pub connections: Vec<Connection>,
}

impl App {
    pub fn new(mut db: SqliteConnection) -> Self {
        let connections = Connection::get_all(&mut db).unwrap_or_default();
        Self {
            input: String::new(),
            cursor_position: 0,
            messages: Vec::new(),
            create_connection_modal: CreateConnectionModal::default(),
            db,
            connections,
        }
    }

    pub fn refresh_connections(&mut self) {
        if let Ok(conns) = Connection::get_all(&mut self.db) {
            self.connections = conns;
        }
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn submit_connection(&mut self) {

        let port = if self.create_connection_modal.port.is_empty() {
            None
        } else {
            self.create_connection_modal.port.parse::<i32>().ok()
        };

        let identity_file = if self.create_connection_modal.identity_file.is_empty() {
            None
        } else {
            Some(self.create_connection_modal.identity_file.as_str())
        };

        let note = if self.create_connection_modal.note.is_empty() {
            None
        } else {
            Some(self.create_connection_modal.note.as_str())
        };

        match NewConnection::create(
            &mut self.db,
            &self.create_connection_modal.name,
            &self.create_connection_modal.username,
            &self.create_connection_modal.hostname,
            port,
            identity_file,
            note,
        ) {
            Ok(conn) => {
                self.messages.push(format!("Created connection: {}", conn.name));
                self.refresh_connections();
            }
            Err(e) => {
                self.messages.push(format!("Error creating connection: {}", e));
            }
        }
        self.close_modal();
    }

    pub fn close_modal(&mut self) {
        self.create_connection_modal.is_open = false;
        self.create_connection_modal.reset();
    }
}
