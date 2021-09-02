use actix::{Addr, Message};
use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::database;

pub mod server;
pub mod routes;
pub mod client;

#[derive(Clone)]
pub struct AppState {
    pub database: Addr<database::DbExecutor>,
    pub server: Addr<server::Server>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientMessage {
    pub fingerprint: String,
    pub event: String,
    pub data: Value
}

impl Message for ClientMessage {
    type Result = ();
}