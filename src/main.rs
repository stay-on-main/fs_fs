use std::io::{Read, Write, Seek, SeekFrom, Result, Error, ErrorKind};

fn main() {
    println!("Hello, world!");
}

struct ClusterStream {

}

impl Read for ClusterStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        todo!();
    }
}

impl Write for ClusterStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        todo!();
    }

    fn flush(&mut self) -> Result<()> {
        todo!();
    }
}

impl ClusterStream {
    fn open(cluster: u32) -> Result<Self> {
        todo!();
    }

    fn create() -> Result<Self> {
        todo!();
    }

    fn close(self) -> Result<()> {
        todo!();
    }

    fn content_cluster(&self) -> u32 {
        todo!();
    }
}

struct DirEntry {
    parrent: Option<u32>,
}

impl DirEntry {
    fn file_new(name: &str, content_cluster: u32, parrent_cluster: u32) -> Result<Self> {
        todo!();
    }

    fn dir_new(name: &str, content_cluster: u32, parrent_cluster: u32) -> Result<Self> {
        todo!();
    }

    fn rename(&mut self, name: &str) -> Result<()> {
        todo!();
    }

    fn update(&self) -> Result<()> {
        todo!();
    }

    fn is_file(&self) -> bool {
        todo!();
    }

    fn is_dir(&self) -> bool {
        todo!();
    }

    fn compare(&self, name: &str) -> bool {
        todo!();
    }

    fn parrent(&self) -> Option<u32> {
        todo!();
    }

    fn cluster(&self) -> u32 {
        todo!();
    }

    fn size(&self) -> u32 {
        todo!();
    }
}

struct File {
    dir_entry: DirEntry,
    stream: ClusterStream,
}

impl File {
    fn new(dir_entry: DirEntry) -> Result<Self> {
        if dir_entry.is_file() {
            let stream = ClusterStream::open(dir_entry.cluster())?;
            Ok(Self {
                dir_entry,
                stream,
            })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Object is not a file"))
        }
    }

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
    stream: ClusterStream,
}

impl DirIterator {
    fn new(cluster: u32) -> Result<Self> {
        let stream = ClusterStream::open(cluster)?;
        Ok(Self { stream })
    }
}

impl Iterator for DirIterator {
    type Item = Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

struct Dir {
    dir_entry: DirEntry,
}

impl Dir {
    fn add(&self, dir_entry: &DirEntry) -> Result<()> {
        todo!();
    }

    fn new(dir_entry: DirEntry) -> Result<Self> {
        if dir_entry.is_dir() {
            Ok(Self { dir_entry })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Object is not a dir"))
        }
    }

    fn find(&self, name: &str) -> Result<DirEntry> {
        for dir_entry in self.iter()? {
            let entry = dir_entry?;
            
            if entry.compare(name) {
                return Ok(entry);
            }
        }

        Err(Error::new(ErrorKind::InvalidInput, "Can't open folder or file"))
    }

    pub fn file_open(&self, name: &str) -> Result<File> {
        let dir_entry = self.find(name)?;
        File::new(dir_entry)
    }

    pub fn file_create(&self, name: &str) -> Result<File> {
        let stream = ClusterStream::create()?;
        let dir_entry = DirEntry::file_new(name, stream.content_cluster(), self.dir_entry.cluster())?;
        self.add(&dir_entry)?;
        File::new(dir_entry)
    }

    pub fn dir_open(&self, name: &str) -> Result<Dir> {
        let dir_entry = self.find(name)?;
        Dir::new(dir_entry)
    }

    pub fn dir_create(&self, name: &str) -> Result<Dir> {
        let stream = ClusterStream::create()?;
        //
        // need to initialize folder
        //
        let dir_entry = DirEntry::dir_new(name, stream.content_cluster(), self.dir_entry.cluster())?;
        self.add(&dir_entry)?;
        Dir::new(dir_entry)
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        todo!();
    }

    pub fn rename(&self, name: &str, new_name: &str) -> Result<()> {
        let mut dir_entry = self.find(name)?;
        dir_entry.rename(new_name)?;
        dir_entry.update()
    }

    pub fn iter(&self) -> Result<DirIterator> {
        DirIterator::new(self.dir_entry.cluster())
    }
}