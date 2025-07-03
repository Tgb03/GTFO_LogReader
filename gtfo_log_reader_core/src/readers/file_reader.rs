use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::PathBuf,
    sync::mpsc::Receiver,
};

pub struct FileReader {
    receiver: Receiver<PathBuf>,

    file: Option<BufReader<File>>,
    last_position: u64,
}

impl FileReader {
    pub fn new(receiver: Receiver<PathBuf>) -> Self {
        Self {
            receiver,
            file: None,
            last_position: 0,
        }
    }

    fn new_file(&mut self, path: PathBuf) {
        self.file = File::open(path).map(|f| BufReader::new(f)).ok();
        self.last_position = 0;
    }

    pub fn get_new_lines(&mut self) -> Option<String> {
        if let Ok(new_path) = self.receiver.try_recv() {
            self.new_file(new_path);
        }

        let Some(reader) = &mut self.file else {
            return None;
        };

        if let Err(_) = reader.seek(std::io::SeekFrom::Start(self.last_position)) {
            return None;
        }

        let mut buffer = String::new();
        let mut line = String::new();

        while reader.read_line(&mut line).unwrap_or(0) > 0 {
            buffer.push_str(&line);
            line.clear();
        }

        self.last_position = reader.stream_position().unwrap_or(self.last_position);

        Some(buffer)
    }
}
