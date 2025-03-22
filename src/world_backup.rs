use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::time::SystemTime;

use crate::expect_exit::ExpectExit;
use chrono::{DateTime, Utc};
use iso8601_timestamp::Timestamp;
use zip_archive::Archiver;
use zip_archive::Format;

pub fn create_world_backup(world_dir: &Path) -> Result<()> {
    println!("Making world backup...");

    let world_name: String = create_world_name(world_dir)?;
    let timestamp: String = create_backup_timestamp();
    let filename: String = format!("{world_name} - {timestamp}.mcworld");

    println!("{filename}");

    let parent_dir: &Path = world_dir
        .parent()
        .expect_exit("Could not extract directory path to world");
    println!("{:?}, {:?}", world_dir, parent_dir);

    create_archive(world_dir, &parent_dir).expect_exit("Could not create archive of world");

    Ok(())
}

fn create_world_name(world_dir: &Path) -> Result<String> {
    let world_name: String = world_dir
        .file_name()
        .expect_exit("Failed to extract world name")
        .to_str()
        .expect_exit("Failed to convert world name to safe string")
        .to_string();
    Ok(world_name)
}

/// YYYY-MM-DD_HH.MM.SS
fn create_backup_timestamp() -> String {
    let timestamp: Timestamp = Timestamp::now_utc();
    let system_time: SystemTime = timestamp.into();
    let datetime: DateTime<Utc> = DateTime::from(system_time);
    let formatted: String = datetime.format("%Y-%m-%d_%H.%M.%S").to_string();
    formatted
}

fn create_archive(world_dir: &Path, output_dir: &Path) -> Result<()> {
    let mut archiver: Archiver = Archiver::new();
    archiver.push(world_dir);
    archiver.set_destination(output_dir);
    archiver.set_thread_count(4);
    archiver.set_format(Format::Zip);

    match archiver.archive() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::new(ErrorKind::Other, format!("{err}"))),
    }
}
