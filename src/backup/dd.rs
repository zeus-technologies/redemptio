use std::{
    fs::File,
    process::{Command, Stdio},
};

use flate2::GzBuilder;

use crate::backup::Backup;

pub(crate) struct DdBackup;

impl Backup for DdBackup {
    fn backup(&self, device: &str, output: &str) -> std::io::Result<()> {
        let dd = Command::new("dd")
            .arg(format!("if={}", device))
            .arg("bs=4M")
            .arg("status=progress")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "Could not capture stdout")
            })?;

        let output_file = File::create(format!("{}.gz", output))?;
        let mut encoder = GzBuilder::new()
            .filename(output)
            .write(output_file, flate2::Compression::default());
        let mut reader = std::io::BufReader::new(dd);

        std::io::copy(&mut reader, &mut encoder)?;

        encoder.finish()?;
        Ok(())
    }
}
