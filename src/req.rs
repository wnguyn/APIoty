use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::{Parser, ValueEnum};






#[derive(Clone, ValueEnum)]
pub enum ApiArgs {
    User,
    Artist,
    Album,
}


#[allow(dead_code)]
pub struct AotySteal {
    pub url: &str,
    pub arg: ApiArgs,
    pub url: &str,
}



/*
pub impl AotySteal {

    /*
     *
     *
    pub fn new(input: &str, arg: ApiArgs) -> Self {
        println!("stealing whole data..."); 
        match arg {


        }

        }

    }
    */






} 
*/
