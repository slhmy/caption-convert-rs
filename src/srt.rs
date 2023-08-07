use serde::{Deserialize, Serialize};

use crate::youtube::YoutubeTimedText;

#[derive(Serialize, Deserialize)]
pub struct SRTElement {
    pub from_ms: u32,
    pub to_ms: u32,
    pub content: String,
}

pub(crate) fn convert_from_youtube_timed_text(timed_text: YoutubeTimedText) -> Vec<SRTElement> {
    let mut srt_elements: Vec<SRTElement> = Vec::new();
    for event in timed_text.events {
        let mut content = String::new();
        if let Some(segs) = event.segs {
            if segs.len() == 1 && segs[0].utf8 == "\n" {
                continue;
            }
            for seg in segs {
                content.push_str(&seg.utf8);
            }
            srt_elements.push(SRTElement {
                from_ms: event.t_start_ms,
                to_ms: event.t_start_ms + event.d_duration_ms,
                content,
            });
        }
    }
    srt_elements
}

fn ms_to_srt_time_string(input_ms: u32) -> String {
    let ms = input_ms % 1000;
    let seconds = (input_ms / 1000) % 60;
    let minutes = (input_ms / (1000 * 60)) % 60;
    let hours = (input_ms / (1000 * 60 * 60)) % 24;
    println!("{} {} {} {}", hours, minutes, seconds, ms);
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, ms)
}

pub(crate) fn create_file(srt_elements: Vec<SRTElement>) {
    let mut srt_file = String::new();
    for (i, srt_element) in srt_elements.iter().enumerate() {
        srt_file.push_str(&format!("{}\n", i + 1));
        srt_file.push_str(&format!(
            "{} --> {}\n",
            ms_to_srt_time_string(srt_element.from_ms),
            ms_to_srt_time_string(srt_element.to_ms)
        ));
        srt_file.push_str(&format!("{}\n", srt_element.content));
        srt_file.push('\n');
    }
    std::fs::write("output.srt", srt_file).expect("Unable to write file");
}
