use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod backup;
mod config;
mod filesystem;

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() {
    init_tracing();
    info!("starting up");
    let fs = filesystem::get_filesystem_type("/").unwrap();
    let backup_manager = backup::get_backup_manager_by_filesystem(&fs);
    backup_manager
        .backup("/dev/nvme0n1", "./backup.img")
        .unwrap();
    println!("Filesystem type: {}", fs);
}
