use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

use tracing::debug;

use crate::filesystem::get_filesystem_type;

use super::Backup;

pub(crate) struct FsArchiverBackup;

impl Backup for FsArchiverBackup {
    fn backup(&self, device: &str, output: &str) -> std::io::Result<()> {
        let mut args = vec![
            String::from("savefs"),
            String::from("-o"),
            String::from(output),
        ];

        let partitions_file = File::open("/proc/partitions")?;
        let partitions = BufReader::new(partitions_file);
        let device_name = device.split('/').last().unwrap();
        for line in partitions.lines().skip(2) {
            let line = line?;
            debug!("reading line: {}", line);
            let partition = line.split_whitespace().last().unwrap();
            if !partition.eq(device_name) && partition.starts_with(device_name) {
                let full_partition = format!("/dev/{}", partition);
                if get_filesystem_type(&full_partition)? != "ext4" {
                    continue;
                }
                debug!("found partition: {}", full_partition);
                args.push(full_partition);
            }
        }

        args.append(&mut vec![
            String::from("-A"),
            String::from("-Z"),
            String::from("9"),
            String::from("-j"),
            String::from("4"),
        ]);
        debug!("running fsarchiver with args: {:?}", args);

        let status = Command::new("fsarchiver").args(args).status()?;

        if status.success() {
            debug!("backup succeeded");
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "fsarchiver failed",
            ))
        }
    }
}
