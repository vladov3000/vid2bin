use std::fs::File;
use std::io::ErrorKind;

pub fn open_file(filename: &str) -> Result<File, &'static str>{
    match File::open(filename) {
        Ok(f) => Ok(f),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return Err("Input file not found"),
            other_err => panic!("Problem opening the file: {:?}", other_err),
        }
    }
}
