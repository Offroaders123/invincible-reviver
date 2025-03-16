use std::io::{Error, ErrorKind, Result};
use std::time::{Duration, SystemTime};

pub fn create_world_backup() -> Result<()> {
    println!("Making world backup...");

    let sys_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    println!("{:?}", sys_time);

    println!("<to be implemented>");

    Ok(())
}
