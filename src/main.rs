use std::sync::Arc;
use std::time::Duration;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use headless_chrome::{Browser, LaunchOptions, Tab};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;
fn main() {
    println!("Hello, world!");
}
