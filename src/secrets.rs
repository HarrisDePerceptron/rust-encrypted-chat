use std::env;
use actix_web::{
    cookie::{time::Duration}
};

use once_cell::sync::Lazy;


pub const ONE_MINUTE: Duration = Duration::minutes(1);

pub const SESSION_KEY: Lazy<String> = Lazy::new(|| env::var("SESSION_KEY").expect("Session key not found"));

pub const TOKEN_ISSUER: Lazy<String> = Lazy::new(|| env::var("TOKEN_ISSUER").expect("TOKEN_ISSUER not found"));
pub const TOKEN_EXPIRY_DAYS: Lazy<String> = Lazy::new(|| env::var("TOKEN_EXPIRY_DAYS").expect("TOKEN_EXPIRY_DAYS not found"));


