
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
async fn main() {
    println!("starting web scraper....");

    /* start thing */
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head()
            .build()?,
    ).await?;
}
