use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
    thread::{self, JoinHandle},
    time::Duration,
};

use might_sleep::prelude::CpuLimiter;

pub struct FolderWatcher {
    sender: Sender<()>,
    join: Option<JoinHandle<()>>,

    update_folder_path: Sender<PathBuf>,
}

impl Drop for FolderWatcher {
    fn drop(&mut self) {
        let _ = self.sender.send(());
        let _ = match self.join.take() {
            Some(jh) => {
                let _ = jh.join();
            }
            None => {}
        };
    }
}

impl FolderWatcher {
    pub fn new_watcher(folder_path: Option<PathBuf>) -> (Receiver<PathBuf>, FolderWatcher) {
        println!("Thread started");

        let (sender, recv) = channel::<PathBuf>();
        let (sender_shutdown, recv_shutdown) = channel::<()>();
        let (sender_path, recv_path) = channel::<PathBuf>();

        let join = thread::spawn(|| Self::watch(folder_path, sender, recv_shutdown, recv_path));

        (
            recv,
            FolderWatcher {
                sender: sender_shutdown,
                join: Some(join),
                update_folder_path: sender_path,
            },
        )
    }

    pub fn update_path(&self, new_path: PathBuf) {
        let _ = self.update_folder_path.send(new_path);
    }

    fn watch(
        mut folder_path: Option<PathBuf>,
        sender: Sender<PathBuf>,
        shutdown: Receiver<()>,
        update_folder_path: Receiver<PathBuf>,
    ) {
        let mut limiter = CpuLimiter::new(Duration::from_secs(1));
        let mut counter: u8 = 0;
        let mut last_metadata = None;

        loop {
            if let Ok(()) = shutdown.try_recv() {
                break;
            }

            if let Ok(path) = update_folder_path.try_recv() {
                folder_path = Some(path);
                last_metadata = None;

                while let Ok(_) = update_folder_path.try_recv() {}

                counter = 10;
            }

            if counter == 10 {
                // not using notify cause of issues with large folders just in case
                let metadata = folder_path
                    .as_ref()
                    .map(|f| fs::read_dir(&f).ok())
                    .flatten()
                    .map(|rd| {
                        rd.flatten()
                            .filter(|f| {
                                let metadata = match f.metadata() {
                                    Ok(metadata) => metadata,
                                    Err(_) => {
                                        return false;
                                    }
                                };

                                metadata.is_file()
                                    && f.file_name()
                                        .to_str()
                                        .unwrap_or_default()
                                        .contains("NICKNAME_NETSTATUS")
                            })
                            .max_by_key(|x| match x.metadata() {
                                Ok(metadata) => metadata.modified().ok(),
                                Err(_) => Default::default(),
                            })
                    })
                    .flatten()
                    .filter(|meta| last_metadata
                        .as_ref()
                        .is_none_or(|lm: &fs::DirEntry| lm.path() != meta.path())
                    );

                if let Some(metadata) = metadata {
                    match sender.send(metadata.path()) {
                        Ok(_) => {}
                        Err(_) => break,
                    }
                    println!("File sent");
                    last_metadata = Some(metadata);
                }

                counter = 0;
            }

            counter += 1;

            limiter.might_sleep();
        }

        println!("Thread ended");
    }
}
