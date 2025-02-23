// use std::fs::OpenOptions;
use std::io::Write;
pub struct CustomLogger {
    file: std::fs::File,
}

impl CustomLogger {
    // pub fn new_from_file_name(log_file: String) -> CustomLogger {
    //     CustomLogger {
    //         file: OpenOptions::new()
    //             .create(true)
    //             .write(true)
    //             .truncate(true)
    //             .open(&log_file)
    //             .unwrap(),
    //     }
    // }
    pub fn new(file: std::fs::File) -> CustomLogger {
        CustomLogger { file }
    }
}

impl Write for CustomLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stdout()
            .write(buf)
            .and_then(|_| self.file.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
