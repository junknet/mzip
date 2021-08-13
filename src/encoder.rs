use std::{
    fmt::Display,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

pub struct Encoder {
    sender: SyncSender<FileData>,
    file_reciver: Receiver<PathBuf>,
}

pub struct FileData {
    name_len: u32,
    name: String,
    data_len: u32,
    data: Vec<u8>,
}

impl FileData {
    pub fn write(&self, file: &mut File) -> std::io::Result<()> {
        file.write(&self.name_len.to_le_bytes())?;
        file.write(self.name.as_bytes())?;
        file.write(&self.data_len.to_le_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}

impl Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "filename: {:?} ,len:{}", self.name, self.name_len)
    }
}

impl Encoder {
    pub fn new(receiver: Receiver<PathBuf>) -> (Self, Receiver<FileData>) {
        let (sender, encode_receiver) = sync_channel(16);
        let encoder = Self {
            file_reciver: receiver,
            sender,
        };
        (encoder, encode_receiver)
    }
    pub fn run(&mut self) {
        for path in &self.file_reciver {
            let file = File::open(path.clone()).unwrap();
            if let Ok(res) = zstd::stream::encode_all(file, 0) {
                let name = String::from(path.to_str().unwrap());
                let file_data = FileData {
                    name_len: name.len() as u32,
                    name,
                    data_len: res.len() as u32,
                    data: res,
                };
                self.sender.send(file_data).unwrap();
            }
        }
    }
}
