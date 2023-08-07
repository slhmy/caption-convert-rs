use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct YoutubeTimedText {
    pub events: Vec<YoutubeEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct YoutubeEvent {
    #[serde(rename = "tStartMs")]
    pub t_start_ms: u32,
    #[serde(rename = "dDurationMs")]
    pub d_duration_ms: u32,
    pub segs: Option<Vec<YoutubeEventSeg>>,
}

#[derive(Serialize, Deserialize)]
pub struct YoutubeEventSeg {
    pub utf8: String,
}

pub fn read_timed_text_from_file(path: PathBuf) -> YoutubeTimedText {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let youtube_timed_text: YoutubeTimedText =
        serde_json::from_str(&contents).expect("failed to parse json");
    youtube_timed_text
}
