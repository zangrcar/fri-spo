use std::fs::OpenOptions;
use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};


pub trait Device: Send {
    fn test(&mut self) -> bool { true }
    fn read(&mut self) -> u8 { 0 }
    fn write(&mut self, _value: u8) {}
}

#[derive(Default)]
pub struct NullDevice;
impl Device for NullDevice {}


#[derive(Default)]
pub struct StdinDevice;
impl Device for StdinDevice {
    fn read(&mut self) -> u8 {
        let mut buf = [0u8;1];
        match io::stdin().read(&mut buf) {
            Ok(1) => buf[0],
            _ => 0,
        }
    }
}

#[derive(Default)]
pub struct StdoutDevice;
impl Device for StdoutDevice {
    fn write(&mut self, value: u8) {
        let _ = io::stdout().write_all(&[value]);
        let _ = io::stdout().flush();
    }
}

#[derive(Default)]
pub struct StderrDevice;
impl Device for StderrDevice {
    fn write(&mut self, value: u8) {
        let _ = io::stderr().write_all(&[value]);
        let _ = io::stderr().flush();
    }
}


pub struct FileDevice {
    file: File,
    pos: u64,
}

impl FileDevice {
    pub fn open_rw(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
        Ok(Self { file, pos: 0 })
    }
}

impl Device for FileDevice {
    fn read(&mut self) -> u8 {
        let mut b = [0u8;1];
        if self.file.seek(SeekFrom::Start(self.pos)).is_ok() {
            if self.file.read(&mut b).unwrap_or(0) == 1 {
                self.pos += 1;
                return b[0];
            }
        }
        0
    }

    fn write(&mut self, value: u8) {
        let _ = self.file.seek(SeekFrom::Start(self.pos));
        if self.file.write_all(&[value]).is_ok() {
            let _ = self.file.flush();
            self.pos += 1;
        }
    }
}