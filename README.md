# Caption Convert in Rust

Currently it's a very basic form of an executable,
many transform into a library if further functionality is needed.

**Now it only read the youtube timedtext in JSON format and convert it into a SRT one**
(ignoring empty segments, and only read and write from `input.json` to `output.srt`)

Please replace the content in `input.json` to get your own srt caption file.