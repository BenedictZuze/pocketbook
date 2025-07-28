use std::process::Command;

#[cfg(target_family = "unix")]
pub fn kill_pid(pid: u32) -> std::io::Result<()> {
    Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .status()?;
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn kill_pid(pid: u32) -> std::io::Result<()> {
    Command::new("taskkill")
        .arg("/PID")
        .arg(pid.to_string())
        .arg("/F")
        .status()?;
    Ok(())
}
