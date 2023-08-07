mod srt;
mod youtube;

fn main() {
    let youtube_timed_text =
        youtube::read_timed_text_from_file(std::path::PathBuf::from("input.json"));
    let srt_elements = srt::convert_from_youtube_timed_text(youtube_timed_text);
    srt::create_file(srt_elements);
}
