use std::error::Error;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn restart_bluetooth() -> Result<(), Box<dyn Error>> {
    Command::new("rfkill")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("block")
        .arg("bluetooth")
        .spawn()?;

    thread::sleep(Duration::from_secs(2));

    Command::new("rfkill")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("unblock")
        .arg("bluetooth")
        .spawn()?;

    Ok(())
}
