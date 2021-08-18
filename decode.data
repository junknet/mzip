use std::{
    path::{Path, PathBuf},
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

use walkdir::WalkDir;

pub struct Explorer {
    sender: SyncSender<PathBuf>,
}

impl Explorer {
    pub fn new() -> (Self, Receiver<PathBuf>) {
        let (sender, receiver) = sync_channel::<PathBuf>(1024);
        let explorer = Self { sender };
        (explorer, receiver)
    }
    pub fn walkdir(&self, path: &Path) {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            self.sender.send(entry.into_path()).unwrap();
        }
    }
}
