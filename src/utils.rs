use std::env;

pub fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

pub fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}