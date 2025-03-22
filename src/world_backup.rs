use std::{io::Result, time::SystemTime};

use chrono::{DateTime, Utc};
use iso8601_timestamp::Timestamp;

pub fn create_world_backup() -> Result<()> {
    println!("Making world backup...");

    let timestamp: Timestamp = Timestamp::now_utc();
    let system_time: SystemTime = timestamp.into();
    let datetime: DateTime<Utc> = DateTime::from(system_time);
    let formatted: String = datetime.format("%Y-%m-%d_%H.%M.%S").to_string();

    println!("{:?}", formatted);

    println!("<to be implemented>");

    Ok(())
}
