use chromiumoxide::browser::Browser;
use chromiumoxide::Page;
use scraper::Html;
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use crate::DurationTime;




pub enum AotyReq 
{
    Album(bool),
    Artist(bool),
    // Chart (do later)
}

enum Album 
{
    GetInfo,
    GetTopTags,
    Search,
}


#[derive(Clone)]
pub struct Engine 
{
    engine: Arc<Mutex<Browser>>,
    pub status: Arc<RwLock<DurationTime>>,
    page: Page,
}

impl Engine 
{


    pub async fn new(brow: Arc<Mutex<Browser>>, enum_status: Arc<RwLock<DurationTime>>) -> Self 
    {
        let shr_page = brow.lock().new_page("about::blank").await.unwrap();
        Self 
        {
            engine: brow,
            status: enum_status,
            page: shr_page,
        }

    }
    pub async fn update_page(&self, url: &str)  -> Html
    {   
        let page_ptr = self.page.clone();       
        page_ptr.goto(url).await.ok();
        let scraped_str = page_ptr.content().await.unwrap();
        
        let return_html = Html::parse_document(&scraped_str);
        return_html

    }

    // return url that represents the actual url for an album
    pub async fn returlfromreq(&self, req_in: &str)  -> String
    {
        println!("returning url to search up an album....");
        let page_ptr = self.page.clone();
        let var = crate::search2url(req_in, page_ptr, true).await.unwrap();
        var
    }
    
        

}
