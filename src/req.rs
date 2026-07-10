use chromiumoxide::browser::Browser;
use chromiumoxide::Page;
use scraper::Html;
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use crate::{DurationTime, UrlWhat};


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
    

    // ret ARTIST URL from generic query
    
    pub async fn returlfromreq(&self, req_in: &str)  -> String
    {
        println!("returning url to search up an album....");
        let page_ptr = self.page.clone();
        let var = crate::search2url(req_in, page_ptr, UrlWhat::UrlLink).await.unwrap();
        var
    }


    pub async fn html_req_page(&self, req_in: &str) -> Html 
    {
        let page_ptr = self.page.clone();

        let page = crate::search2url(req_in, page_ptr, UrlWhat::PageData).await.unwrap();
        let html_parse = Html::parse_document(&page);
        return html_parse;
        



    }
    
        

}
