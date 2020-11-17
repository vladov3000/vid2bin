use std::fs::File;
use gif;
use std::vec::Vec;

pub fn draw_ascii(file: File) {
    println!("Playing gif...");
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options.read_info(file).unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let small_frame = scale(&(frame.buffer), frame.width, 70);
        println!("{}", frame_to_ascii(&small_frame, ".:!*%$@&#SB", 70));
    }
}

fn pixel_to_grayscale(r: u8, g: u8, b: u8, a: u8) -> f32 {
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) * a
    as f32 / 255.0 / 255.0
}

fn frame_to_ascii(frame: &[u8], palette: &str, width: u16) -> String {
    let mut res = String::new();
    for i in (3..frame.len()).step_by(4) {
        let g = pixel_to_grayscale(frame[i-3], frame[i-2], frame[i-1], frame[i]);
        for (j, c) in palette.chars().enumerate() {
            if (j as f32 / ((palette.len() - 1) as f32)) > g {
                res.push(c);
                break;
            }
        }
        if i > 0 && i % (width as usize) == (width - 1) as usize {
            res.push('\n');
        }
    }

    res
}

fn scale(frame: &[u8], width: u16, new_width: u16) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let width = width as f32;
    let new_height = (frame.len() as f32 / width * new_width as f32 / width).ceil() as u16;
    let chunkWidth = (width / new_width as f32).ceil() as u16;
    let chunkHeight = ((frame.len() as f32 / width) / new_height as f32).ceil() as
    u16;
    for i in 0..new_height*new_width-1 {
        let mut new_rgba = (0 as u32, 0 as u32, 0 as u32, 0 as u32);
        let mut count = 0;
        for j in 0..chunkHeight {
            for k in (3..chunkWidth).step_by(4) {
                count += 1;
                new_rgba.0 += frame[k as usize -3 + j as usize] as u32;
                new_rgba.1 += frame[k as usize -2 + j as usize] as u32;
                new_rgba.2 = frame[k as usize -1 + j as usize] as u32;
                new_rgba.3 = frame[k as usize + j as usize] as u32;
            }
        }
        
        res.push((new_rgba.0 / count) as u8);
        res.push((new_rgba.1 / count) as u8); 
        res.push((new_rgba.2 / count) as u8);
        res.push((new_rgba.3 / count) as u8);
    }
    res
}