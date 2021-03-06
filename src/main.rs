pub mod decoder;
mod encoder;
use std::{
    fs::File,
    path::{Path, PathBuf},
    thread::spawn,
};

use decoder::read_file;
use encoder::Encoder;
use explorer::Explorer;

mod explorer;

fn main() {
    let (explorer, receiver) = Explorer::new();
    let (mut encoder, encoder_reciver) = Encoder::new(receiver);
    spawn(move || {
        explorer.walkdir(Path::new(
            &std::env::args().nth(1).unwrap_or("./src".to_string()),
        ))
    });
    spawn(move || encoder.run());
    let mut res_file = File::create("./compressed.data").unwrap();
    for encoded_data in encoder_reciver {
        let _ = encoded_data.write(&mut res_file);
    }
}

#[test]
fn name_len() {
    read_file(PathBuf::from("./compressed.data"));
}
