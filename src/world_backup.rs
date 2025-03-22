use std::io::Result;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use iso8601_timestamp::Timestamp;

pub fn create_world_backup(world_name: &str) -> Result<()> {
    println!("Making world backup...");

    let timestamp: String = create_backup_timestamp();
    let filename: String = format!("{world_name} - {timestamp}.mcworld");

    println!("{filename}");

    println!("<to be implemented>");

    Ok(())
}

/// YYYY-MM-DD_HH.MM.SS
fn create_backup_timestamp() -> String {
    let timestamp: Timestamp = Timestamp::now_utc();
    let system_time: SystemTime = timestamp.into();
    let datetime: DateTime<Utc> = DateTime::from(system_time);
    let formatted: String = datetime.format("%Y-%m-%d_%H.%M.%S").to_string();
    formatted
}
