use scraper::{Html, Selector};
use axum::{Router, routing::get};

use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

use crate::req::{Engine, AotyReq};

use std::sync::{Arc, Mutex};

mod req;
mod entry;
const AOTY: &str = "https://www.albumoftheyear.org/search/?q=";

enum AlbumReq {
    Score,
    Genre,
//    Label,
    ReleaseDt,
}

// wrapper
fn fmt_url(id: &str) -> String 
{   
    let var = format!("{}{}", AOTY, id);
    var.replace(' ', "+")
}


pub trait PrintResp {
    fn print_resp(&self) -> String;
}



// this returns random aoty search query -> first album/whatever. 
pub async fn search2url(args: &str, page: chromiumoxide::Page, pick_album: bool) -> Option<String>
{   

    let url: String = fmt_url(args);
    page.goto(&url).await.ok()?;
    let search_html = page.content().await.ok()?;

    let html_page = Html::parse_document(&search_html);


    match pick_album { // if false attempts to search artist (i'm not supporting user searchup)
      true => {
            let foo = html_page                                                                                                                                                      
               .select(&Selector::parse(".albumBlock .image a").unwrap())                                                                                                                                                   
               .next()                                                                                                                                                                                                      
               .and_then(|el| el.value().attr("href").map(String::from));
            return foo;
        }
        _ => { 
            let foo = html_page                                                                                                                                                      
               .select(&Selector::parse(".artistBlock.image a").unwrap())                                                                                                                                                   
               .next()                                                                                                                                                                                                      
               .and_then(|el| el.value().attr("href").map(String::from));
            return foo;
        }

        
    }
}


#[allow(unused)]
async fn handle_album_req(
    arg: AlbumReq, 
    b: chromiumoxide::Page, 
    url: &str
    ) -> Option<()>
{
    
    let arg_str: String = search2url(url, b.clone(), true).await.unwrap();
    b.goto(&arg_str);
    let search_content = b.content().await.ok().unwrap();


    let formatted_html = Html::parse_document(&search_content);

    let mut res: String;


    match arg 
    {
        AlbumReq::Score => 
        {
             let score = {                                                                                                                                                                                                
               let c: u32 = formatted_html
                   .select(&Selector::parse(".albumCriticScore a").unwrap())                                                                                                                                            
                   .next()?                                                                                                                                                                                             
                   .inner_html()                                                                                                                                                                                        
                   .trim()                                                                                                                                                                                              
                   .parse()                                                                                                                                                                                             
                   .ok()?;                                                                                                                                                                                              
               let u: u32 = formatted_html 
                   .select(&Selector::parse(".albumUserScore a").unwrap())                                                                                                                                              
                   .next()?                                                                                                                                                                                             
                   .inner_html()                                                                                                                                                                                        
                   .trim()                                                                                                                                                                                              
                   .parse()                                                                                                                                                                                             
                   .ok()?;                                                                                                                                                                                              
               (c, u) 
            };
            println!("{:?}", score);
            Some(())
        }
        AlbumReq::Genre => 
        {
             let selector = Selector::parse(".detailRow").unwrap();
             let a_selector = Selector::parse("a").unwrap();
             let mut detail_rows = formatted_html.select(&selector);
             let genres: Vec<String> = detail_rows
               .find(|row| row.inner_html().contains("/&nbsp;Genre"))
               .map(|row| {
                   row.select(&a_selector)
                       .map(|a| a.inner_html().trim().to_string())
                       .collect()
               })?;
             println!("{:?}", genres);
             Some(())
        }
        AlbumReq::ReleaseDt => 
        {
             let date: String = formatted_html
               .select(&Selector::parse(".detailRow").unwrap())
               .find(|row| row.inner_html().contains("/&nbsp;Release Date"))
               .map(|row| {
                   let html = row.inner_html();
                   let end = html.find("/&nbsp;Release Date").unwrap();
                   html[..end].trim().to_string()
               })?;
             println!("{:?}", date);
                Some(())
        }
    }
}


async fn dynamic_handler() -> Json<Value {
    


}



#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{       

    println!("starting chromium oxide....");
    let (browser, mut handler) = Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
    let handle = tokio::spawn(async move {
         while let Some(j) = handler.next().await {
            if j.is_err() {
                break;
            }
         }
    });



    // Page is thread safe so just clone it or whatever yeah
    let shr_page = browser.new_page("about::blank").await.ok().unwrap();
    
    let engine: Engine = Engine::new(
        Arc::new(Mutex::new(browser)),
        None,
    ).await;


    /* Testing "Leaves Turn Inside you" to return data or whatever
    let _str: String = engine.returlfromreq(AotyReq::Artist(true), input_tst).await;
    let var = engine.update_page(&_str).await;
    */

    let app = Router::new()
        .route("/api/v0/:reqwest", get(dynamic_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
                                                              
    Ok(())



}

