use std::env;
use actix_web::{
    cookie::{time::Duration}
};

use once_cell::sync::Lazy;

pub const ONE_MINUTE: Duration = Duration::minutes(1);

pub static SESSION_KEY: Lazy<String> = Lazy::new(|| env::var("SESSION_KEY").expect("Session key not found"));

