extern crate memmap;
use self::memmap::Mmap;
use std::fs::File;

pub struct FastFileSource {
    mmap: Mmap,
}

impl FastFileSource {
    pub fn new(filename: &str) -> std::io::Result<FastFileSource> {
        let file = File::open(filename)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let ffs = FastFileSource { mmap: mmap };

        Ok(ffs)
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, u8> {
        self.mmap.as_ref().iter()
    }
}
