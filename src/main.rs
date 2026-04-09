mod app;
mod db;
mod ui;
mod input;

use app::App;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crossterm::event::{self, Event};

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let db = db::establish_connection();
    let mut app = App::new(db);

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        if let Event::Key(key) = event::read()? {
            if input::handle_key_event(&mut app, key) {
                break;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
