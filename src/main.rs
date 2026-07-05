use axum::handler;
use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::{Parser, ValueEnum};
use futures::StreamExt;
use std::sync::Arc;
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

async fn search2url(args: &strm, b: &browser) -> Option<String>
{   
    let url: String = fmt_url(args);
    let page = b.new_page("about:blank");
    page.goto(&url).await?;
    let search_html = page.content().await?;

    let html_page = Html::parse_document(search_html);

    html_page                                                                                                                                                      
       .select(&Selector::parse(".albumBlock .image a").unwrap())                                                                                                                                                   
       .next()                                                                                                                                                                                                      
       .and_then(|el| el.value().attr("href").map(String::from))
}


#[allow(unused)]
async fn handle_album_req(
    arg: AlbumReq, 
    b: &chromiumoxide::Browser, 
    h: &chromiumoxide::Handler,
    url: &str
    ) -> Result<(), Box<dyn std::error::Error>>
{

    
    let arg_str = search2url(url, b).await;
    let mut res: String;
    match arg_str {
        Some(str) => {
            res = str;
        }
        _ => {
            println!("failed to get entry");
            None(())
        }
    };

    let page = b.new_page("about:blank").await?;
    page.goto(
    match &arg 
    {
        AlbumReq::Score => 
        {
            b.new_page(arg_str).await?;



            

        }
        AlbumReq::Genre => 
        {


        }
        AlbumReq::ReleaseDt => 
        {




        }
    }

    Ok(())
}




#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    println!("starting chromium oxide....");
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
    let handle = tokio::spawn(async move {
         while let Some(j) = handler.next().await {
            if j.is_err() {
                break;
            }
         }
    });

    





    Ok(())



}

