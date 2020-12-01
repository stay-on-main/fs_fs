use std::io::{Read, Write, Seek, SeekFrom, Result, Error, ErrorKind};

fn main() {
    println!("Hello, world!");
}


struct DirEntry {
    
}

struct File {
    dir_entry: DirEntry,
}

impl File {
    fn close(self) -> Result<()> {
        todo!();
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        todo!();
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        todo!();
    }

    fn flush(&mut self) -> Result<()> {
        todo!();
    }
}

struct DirIterator {

}

impl DirIterator {
    fn next(&mut self) -> Option<Result<DirEntry>> {
        todo!();
    }
}

struct Dir {
    dir_entry: DirEntry,
}

impl Dir {
    pub fn file_open(&self, file_name: &str) -> Result<File> {
        todo!();
    }

    pub fn file_create(&self, file_name: &str) -> Result<File> {
        todo!();
    }

    pub fn dir_open(&self, dir_name: &str) -> Result<Dir> {
        todo!();
    }

    pub fn dir_create(&self, dir_name: &str) -> Result<Dir> {
        todo!();
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        todo!();
    }

    pub fn rename(&self, name: &str, new_name: &str) -> Result<()> {
        todo!();
    }

    pub fn iter(&mut self) -> Result<DirIterator> {
        todo!();
    }
}