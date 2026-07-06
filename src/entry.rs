



pub struct AlbumGetInfo {
    name: String,
    artist: String,
    // mbid
    url: String,
    critic_score: u8,
    user_score: u8,
    tracks: Vec<Song>,
    tags: Vec<String>,
}

pub struct Song {
    name: String,
    duration: u32,
    artist: String,
}
