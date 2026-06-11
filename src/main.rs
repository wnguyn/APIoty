use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::{Parser, ValueEnum};
use futures::StreamExt;
mod req;
use req::AotySteal;
use req::ApiArgs;
const AOTY: &str = "https://www.albumoftheyear.org/search/?q=";

#[derive(Parser, Clone)]
pub struct Args {
    #[arg(short, long)]
    pub url: String,

    #[arg(value_enum)]
    pub count: ApiArgs,
    
}


pub fn fmt_url(fmt: &str) -> String {       
    let unfmt = AOTY + fmt;
    let fmt: String = unfmt.replace(" ", "+");

    fmt

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
    let reqwest: AotySteal =  AotySteal {
            url: args.url,
            arg: args.count,
            fmt: fmt_url(args.url),

    };
    match &reqwest.arg {
       ApiArgs::Artist => {
           browser.new_page(&
           




       }
       ApiArgs::User => {



       }
       ApiArgs::Album => {





       }



    }
    Ok(())


}
