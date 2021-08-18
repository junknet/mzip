use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};


pub fn read_file(path: PathBuf) {
    let mut file = File::open(path).unwrap();
    let mut name_len: [u8; 4] = [0; 4];
    file.read_exact(&mut name_len).unwrap();
    let name_len = u32::from_le_bytes(name_len) as usize;
    let mut name: Vec<u8> = vec![0; name_len];
    file.read_exact(&mut name).unwrap();
    let name = String::from_utf8_lossy(&name);
    println!("name: {}", name);
    let mut data_len: [u8; 4] = [0; 4];
    file.read_exact(&mut data_len).unwrap();
    let data_len = u32::from_le_bytes(data_len) as usize;
    let mut data: Vec<u8> = vec![0; data_len];
    file.read_exact(&mut data).unwrap();

    let data = zstd::stream::decode_all(data.as_slice()).unwrap();
    let mut decode_data = File::create("decode.data").unwrap();
    decode_data.write(&data).unwrap();
}
