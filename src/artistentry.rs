use scraper::{Html, Selector};
use serde::Serialize;

fn sel(s: &str) -> Selector {
    Selector::parse(s).expect("bad selector")
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RelType {
    LP,
    EP,
    Single,
    Live,
    Demo,
    Compilation,
    Reissue,
    BoxSet,
    Miscellaneous,
    Other(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct AlbumEntry {
    pub title: String,
    pub url: String,
    pub year: String,
    pub release_type: RelType,
    pub artists: Vec<String>,
    pub critic_score: Option<u8>,
    pub usr_score: Option<u8>,
}

impl AlbumEntry {
    
    // html from engine interface
    pub fn new(html_data: Html) -> Vec<AlbumEntry> {
        html_data
            .select(&sel(".albumBlock.small"))
            .map(|block| {
                let url = block
                    .select(&sel("a[href^='/album/']"))
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let title = block
                    .select(&sel(".albumTitle.normal"))
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .unwrap_or_default();

                // ".type" text: "2001 • LP" or "2024 • Reissue" or "1993 • Single • <span>Artists</span>"
                let type_el = block.select(&sel(".type")).next();
                let type_text = type_el
                    .map(|el| {
                        el.text()
                            .collect::<String>()
                    })
                    .unwrap_or_default();

                let (year, release_type) = Self::parse_type_str(&type_text);

                // Featured artists from <span> child of .type
                let artists: Vec<String> = type_el
                    .and_then(|el| el.select(&sel("span")).next())
                    .map(|span| {
                        span.text()
                            .collect::<String>()
                            .split(", ")
                            .map(|s| s.trim().to_string())
                            .collect()
                    })
                    .unwrap_or_default();

                // Score rows — each .ratingRow has a score and a label
                let mut critic_score = None;
                let mut usr_score = None;
                for row in block.select(&sel(".ratingRow")) {
                    let score: u8 = row
                        .select(&sel(".ratingBlock .rating"))
                        .next()
                        .and_then(|el| el.text().collect::<String>().trim().parse().ok())
                        .unwrap_or(0);
                    let label = row
                        .select(&sel(".ratingText"))
                        .next()
                        .map(|el| el.text().collect::<String>())
                        .unwrap_or_default();
                    if label.contains("critic") {
                        critic_score = Some(score);
                    } else if label.contains("user") {
                        usr_score = Some(score);
                    }
                }

                AlbumEntry {
                    title,
                    url,
                    year,
                    release_type,
                    artists,
                    critic_score,
                    usr_score,
                }
            })
            .collect()
    }

    fn parse_type_str(s: &str) -> (String, RelType) {
        let parts: Vec<&str> = s.split(" • ").collect();
        if parts.len() < 2 {
            return (String::new(), RelType::Other(s.to_string()));
        }
        let year = parts[0].to_string();
        let type_name = parts[1].trim();
        let rel = match type_name {
            "LP" => RelType::LP,
            "EP" => RelType::EP,
            "Single" => RelType::Single,
            "Live" => RelType::Live,
            "Demo" => RelType::Demo,
            "Compilation" => RelType::Compilation,
            "Reissue" => RelType::Reissue,
            "Box Set" => RelType::BoxSet,
            "Miscellaneous" => RelType::Miscellaneous,
            other => RelType::Other(other.to_string()),
        };
        (year, rel)
    }
}
