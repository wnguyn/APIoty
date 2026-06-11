use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::Parser;
use futures::StreamExt;
mod req;
use req::AotySteal;

#[derive(Parser, Clone)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    count: usize,
}


#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting scraper");
    let args = Args::parse();
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    /* A primative CLI before I try to deploy this as a REST API on my own website */
    let page = browser.new_page(&args.url).await?;
    let reqwest: AotySteal

    

    



    Ok(())


}
