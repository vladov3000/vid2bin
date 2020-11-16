use std::fs::File;
use std::io::ErrorKind;
use gif;

pub fn play_gif(file: File) {
    println!("Playing gif...");
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options.read_info(file).unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        println!("{} {}", frame.height, frame.width);
    }
}
