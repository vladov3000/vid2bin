use vid2bin::{play, utils};
use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("vid2bin")
        .version("1.0")
        .author("Vladimir O. <vladov3000@gmail.com> Robert B. <roblburris@gmail.com>")
        .about("Plays gifs in the terminal")
        .args(&[
            Arg::new("input")
                .about("the input file to use")
                .required(true),
        ])
        .get_matches();

    let file = utils::open_file(matches.value_of("input").unwrap())?;
    play::play_gif(file);

    Ok(())
}
