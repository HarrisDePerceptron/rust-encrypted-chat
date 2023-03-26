use uuid::Uuid;

use std::time::{Duration, UNIX_EPOCH, SystemTime, SystemTimeError};


pub fn generate_unique_id() -> Result<String, SystemTimeError> {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH)?.as_secs();
    println!("{}", timestamp);
    let random_id = Uuid::new_v4().simple().to_string();
    let uid  = format!("{}-{}", timestamp, random_id);
    return Ok(uid);
}
