use crate::db::connection::Connection;
use crate::db::history::History;
use chrono::Utc;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::process::Command;

pub fn execute_ssh_connection(
    conn: &Connection,
    db: &mut diesel::SqliteConnection,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), std::io::Error> {
    // 1. Temporarily yield terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;

    // Record start time
    let start_time = Utc::now().timestamp() as i32;

    // 2. Build ssh command
    let mut cmd = Command::new("ssh");

    if let Some(port) = conn.port {
        cmd.arg("-p").arg(port.to_string());
    }

    if let Some(ref identity) = conn.identity_file {
        cmd.arg("-i").arg(identity);
    }

    let user_host = format!("{}@{}", conn.username, conn.hostname);
    cmd.arg(user_host);

    // 3. Execute and wait - the child process inherits stdin/out/err
    let status = cmd.status()?;

    // 4. Record history
    let end_time = Utc::now().timestamp() as i32;
    let exit_code = if status.success() {
        "0".to_string()
    } else {
        match status.code() {
            Some(code) => code.to_string(),
            None => "unknown".to_string(),
        }
    };

    if let Some(id) = conn.id {
        let _ = History::create(db, id, start_time, end_time, exit_code);
    }

    // 5. Restore terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen
    )?;
    terminal.clear()?;

    Ok(())
}
