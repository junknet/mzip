use std::{
    fmt::Display,
    fs::File,
    io::Write,
    os::unix::prelude::OsStringExt,
    path::PathBuf,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

use threadpool::ThreadPool;

pub struct Encoder {
    sender: SyncSender<FileData>,
    file_reciver: Receiver<PathBuf>,
    pool: ThreadPool,
}

pub struct FileData {
    name_len: u32,
    name: Vec<u8>,
    data_len: u32,
    data: Vec<u8>,
}

impl FileData {
    pub fn write(&self, file: &mut File) -> std::io::Result<()> {
        file.write(&self.name_len.to_le_bytes())?;
        file.write(&self.name)?;
        file.write(&self.data_len.to_le_bytes())?;
        file.write(&self.data)?;
        //  太耗时
        // println!("filename:{}", self.name.as_str());
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
        let pool = ThreadPool::new(num_cpus::get());
        let encoder = Self {
            file_reciver: receiver,
            sender,
            pool,
        };
        (encoder, encode_receiver)
    }
    pub fn run(&mut self) {
        for path in &self.file_reciver {
            let tx = self.sender.clone();
            self.pool.execute(move || {
                if let Ok(file) = File::open(path.clone()) {
                    if let Ok(res) = zstd::stream::encode_all(file, 0) {
                        // if let Some(path) = path.to_str() {
                        let name = path.into_os_string().into_vec();
                        let file_data = FileData {
                            name_len: name.len() as u32,
                            name,
                            data_len: res.len() as u32,
                            data: res,
                        };
                        tx.send(file_data).unwrap();
                        // } else {
                        //     println!("path convert failed: {:?}", path);
                        // }
                    }
                }
            })
        }
    }
}
