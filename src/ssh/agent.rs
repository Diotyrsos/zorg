use std::process::Command;
use std::path::Path;

pub fn is_agent_running() -> bool {
    std::env::var("SSH_AUTH_SOCK").is_ok()
}

pub fn add_key_to_agent(key_path: &Path) -> std::io::Result<bool> {
    let status = Command::new("ssh-add")
        .arg(key_path)
        .status()?;
    Ok(status.success())
}
