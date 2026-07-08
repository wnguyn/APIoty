use scraper::{Selector, Html};
use std::time::Duration;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Resp {
    resp: AlbumData
}


pub trait PrintResp 
{
    fn print_resp(&self) -> String;
}


#[derive(Clone, Serialize)]
pub struct AlbumData 
{
    #[serde(skip)]
    web_data: Html,

    name: String,
    artist: String,
    url: String,
    cover_url: String,
    critic_score: u8,
    user_score: u8,
    tracks: Vec<Song>,
    tags: Vec<String>,
}


#[derive(Clone, Serialize)]
pub struct Song 
{
    name: String,
    duration: u32,
    artist: String,
}


fn sel(s: &str) -> Selector 
{
    Selector::parse(s).expect("NaN")
}


fn parse_duration(s: &str) -> Option<u32> 
{
   let (mins, secs) = s.split_once(':')?;
   Some(mins.parse::<u32>().ok()? * 60 + secs.parse::<u32>().ok()?)
}



impl AlbumData {

    // TODO: Maybe not use unwrap for all these....
    pub fn init_html(&self, album_html: Html) -> Self {

        let alb_name = album_html
            .select(&sel("h1.albumTitle"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        let artist = album_html
            .select(&sel(".artist .a"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();
         
        let url = album_html
            .select(&sel("link[rel=\"canonical\"]"))
            .next()
            .and_then(|el|el.value().attr("href"))
            .unwrap()
            .to_string();

        let critic_score: u8 = album_html
            .select(&sel(".albumCriticScore a"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .parse()
            .unwrap();

        let user_score: u8 = album_html
            .select(&sel(".albumUserScore a"))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .parse().unwrap();

        let mut tracks = Vec::new();
        for table in album_html.select(&sel("table.trackListTable")) 
        {   
            for row in table.select(&sel("tr")) 
            {
                let name = row
                    .select(&sel("td.trackTitle a"))
                    .next()
                    .map(|a|a.text().collect::<String>().trim().to_string());
                let duration = row
                    .select(&sel("td.trackTitle .length"))
                    .next()
                    .map(|d|parse_duration(d.text().collect::<String>().trim()))
                    .unwrap();
                if let (Some(name), Some(duration)) = (name, duration) 
                {
                    tracks.push(Song {
                        name,
                        duration,
                        artist: artist.clone()
                    });
                }
            }

        }

        let tags: Vec<String> = album_html
            .select(&sel(".tag a"))
            .map(|a| a.text().collect::<String>().trim().to_string())
            .collect();

        // fetch cover art url 
        //
        let cover_url = album_html
            .select(&sel(".albumTopBox.cover.img"))
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        

        Self {
            web_data: album_html,
            name: alb_name,
            artist,
            url,
            critic_score,
            user_score,
            tracks,
            cover_url,
            tags,
        }



    }




}

impl PrintResp for AlbumData {

    // for console before I add api on top
    fn print_resp(&self) -> String 
    {   
        let mut track_str = String::new();
        let mut tag_str = String::new();

        for track in &self.tracks 
        {
            let name = &track.name;
            let dur = track.duration;
            let artist = &track.artist;


            let string_track= format!("
                song_name: {name} \n
                dur: {dur} \n
                artist: {artist} \n");
            track_str.push_str("{string_track}");
        }

        
        let mut genres = String::new();
        for genre in &self.tags
        {
            let tag = format!(" {{&genre.tag}} ");
            genres.push_str("{tag}");
            
        }
        let tag_str = format!(" {{&genres}} ");

        let name = &self.name;
        let _var = format!(
            "album name: {name} \n
             artist name: {{&self.artist}} /n
             url: {{&self.url}} \n
             critic score: {{&self.critic_score}} \n
             user score = {{&self.user_score}} \n
             tracks = {track_str} \n
             tags = {tag_str} \n
        ");
        _var
    }

}
