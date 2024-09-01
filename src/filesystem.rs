#[cfg(target_family = "unix")]
pub fn get_filesystem_type(path: &str) -> std::io::Result<String> {
    use std::process::Command;

    let output = Command::new("df").args(&["-T", path]).output()?;
    if output.status.success() {
        let output = String::from_utf8_lossy(&output.stdout);
        let filesystem_type = output
            .lines()
            .nth(1)
            .and_then(|line| line.split_whitespace().nth(1))
            .map(|s| s.to_string())
            .expect("failed to parse output of df, cannot determine filesystem type");
        Ok(filesystem_type.to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "df command failed",
        ))
    }
}

#[cfg(target_family = "windows")]
pub fn get_filesystem_type(path: &str) -> std::io::Result<String> {
    todo!()
}
