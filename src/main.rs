mod encoder;
use std::{fs::File, path::Path, thread::spawn};

use encoder::Encoder;
use explorer::Explorer;

mod explorer;

fn main() {
    let (explorer, receiver) = Explorer::new();
    let (mut encoder, encoder_reciver) = Encoder::new(receiver);
    spawn(move || explorer.walkdir(Path::new("/home/junknet/Downloads/enwik8")));
    spawn(move || encoder.run());
    let mut res_file = File::create("./res.data").unwrap();
    for encoded_data in encoder_reciver {
        let _ = encoded_data.write(&mut res_file);
    }
}
