use scraper::{Html, Selector};
use serde::Serialize;

fn sel(s: &str) -> Selector {
    Selector::parse(s).expect("bad selector")
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchAlbum {
    pub title: String,
    pub artist: String,
    pub artist_url: String,
    pub album_url: String,
    pub year: String,
    pub release_type: String,
    pub is_must_hear: bool,
    pub image: String,
}



pub fn parse_search(html: &Html) -> (Vec<SearchAlbum>, Vec<String>) {
    let albums = html
        .select(&sel(".albumBlock"))
        .map(|block| {
            let album_url = block
                .select(&sel("a[href^='/album/']"))
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            let artist = block
                .select(&sel(".artistTitle"))
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let artist_url = block
                .select(&sel("a[href^='/artist/']"))
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            let title = block
                .select(&sel(".albumTitle"))
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let type_text = block
                .select(&sel(".type"))
                .next()
                .map(|el| {
                    el.text()
                        .collect::<String>()
                })
                .unwrap_or_default();
            let (year, release_type) = type_text
                .split_once(" • ")
                .map(|(y, t)| (y.to_string(), t.to_string()))
                .unwrap_or_default();

            let is_must_hear = block.select(&sel(".mustHear")).next().is_some();

            let image = block
                .select(&sel("img"))
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            SearchAlbum {
                title,
                artist,
                artist_url,
                album_url,
                year,
                release_type,
                is_must_hear,
                image,
            }
        })
        .collect();

    let artists: Vec<String> = html
        .select(&sel(".artistBlock.six .name"))
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    (albums, artists)
}
