pub mod main_handler;
pub mod modal_handler;

use crate::app::App;
use crossterm::event::KeyEvent;
use main_handler::handle_main_input;
use modal_handler::handle_modal_input;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    if app.create_connection_modal.is_open {
        handle_modal_input(app, key)
    } else {
        handle_main_input(app, key)
    }
}
