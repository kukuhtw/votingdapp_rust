/*
backend/src/lib.rs
*/
pub mod util;
pub mod db;
pub mod routes;
pub mod models;
pub mod security;
pub mod middleware;
pub mod cosm;
pub mod state;            // <-- add

pub use state::AppState;  // <-- re-export so `crate::AppState` works in lib