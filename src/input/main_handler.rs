use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_main_input(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => app.submit_message(),
        KeyCode::Char(key_pressed) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key_pressed == 'c' {
                return true; // Request exit
            } else if key.modifiers.contains(KeyModifiers::CONTROL) && key_pressed == 'n' {
                app.create_connection_modal.is_open = true;
            } else {
                app.input.push(key_pressed);
            }
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Esc => {
            return true; // Request exit
        }
        _ => {}
    }
    false
}
