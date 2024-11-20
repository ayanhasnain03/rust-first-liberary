#![allow(dead_code, unused_variables)]
use rand::prelude::*;
mod database;
mod auth_utils;

// Use the Credentials struct from auth_utils module
pub use auth_utils::model::Credentials;

// Import Status from database module
use database::Status::Connected;

pub fn authenticate(cred: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("Authenticating user... ({}ms)", timeout);
    
    // Check if the database connection is successful
    if let Connected = database::connect_to_database() {
        // If connected, attempt to login with the provided credentials
        auth_utils::login(cred);
    }
}
