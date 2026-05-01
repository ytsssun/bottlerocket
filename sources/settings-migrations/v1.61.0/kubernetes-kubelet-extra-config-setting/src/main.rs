use migration_helpers::common_migrations::AddSettingsMigration;
use migration_helpers::{migrate, Result};
use std::process;

/// Added new kubernetes setting for kubelet-extra-config.
fn run() -> Result<()> {
    migrate(AddSettingsMigration(&[
        "settings.kubernetes.kubelet-extra-config",
    ]))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
