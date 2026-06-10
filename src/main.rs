
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

use chromiumoxide::{Browser, BrowserConfig, cdp::browser_protocol::BrowserContextId};






#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error:Error>> {
    println!("starting web scraper....");
    let browser = Browser::new(
        BrowserConfig::builder()
            .with_head()
            .build()
            .unwrap()
    )?;
    Ok(())


}
