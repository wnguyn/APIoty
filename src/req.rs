use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::{Parser, ValueEnum};
use chromiumoxide::Page;
use std::sync::{Arc, Mutex};




enum AotyReq {
    Album,
    Artist,
    // Chart (do later)
}

enum Album {
    GetInfo,
    GetTopTags,
    Search,
}


struct Engine {
    engine: Arc<Mutex<Browser>>,
    page: Page,
    query: String,
    req_type: AotyReq,
}

impl Engine {


    pub async fn new(brow: Arc<Mutex<Browser>>, args: &str, req: AotyReq) -> Self {
        let shr_page = brow.lock().unwrap().new_page("about::blank").await.unwrap();
        Self {
            engine: brow,
            page: shr_page,
            query: format!("{args}"),
            req_type: req,
        }

    }
    
    pub async fn update_page(&self) {
        self->page





    }

}
