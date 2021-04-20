use tokio::sync::Mutex;
use std::sync::Arc;
use carapax::Api;

pub mod config;
pub mod corona;
pub mod memory;
pub mod sources;
pub mod commands;

pub struct Context {
    pub api: Api,
    pub count: Arc<Mutex<i32>>,
}


