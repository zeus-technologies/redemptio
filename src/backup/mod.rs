use tracing::debug;

mod dd;
mod fsarchiver;

pub(crate) trait Backup {
    fn backup(&self, device: &str, output: &str) -> std::io::Result<()>;
}

#[allow(dead_code)]
pub enum BackupStrategy {
    Dd,
    FsArchiver,
    Lvm,
    Vss,
    Zfs,
}

pub fn get_backup_manager_by_filesystem(fs: &str) -> Box<dyn Backup> {
    match fs {
        "ext4" => {
            if command_exists("fsarchiver") {
                debug!("using fsarchiver");
                get_backup_manager(BackupStrategy::FsArchiver)
            } else {
                debug!("fsarchiver not available on system, using dd");
                get_backup_manager(BackupStrategy::Dd)
            }
        }
        _ => unimplemented!(),
    }
}

fn get_backup_manager(strategy: BackupStrategy) -> Box<dyn Backup> {
    match strategy {
        BackupStrategy::Dd => Box::new(dd::DdBackup {}),
        BackupStrategy::FsArchiver => Box::new(fsarchiver::FsArchiverBackup {}),
        BackupStrategy::Lvm => unimplemented!(),
        BackupStrategy::Vss => unimplemented!(),
        BackupStrategy::Zfs => unimplemented!(),
    }
}

fn command_exists(command: &str) -> bool {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(':') {
            if std::path::Path::new(&format!("{}/{}", p, command)).exists() {
                return true;
            }
        }
    }
    false
}
