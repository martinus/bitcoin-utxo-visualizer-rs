// Copyright 2016 Martin Ankerl.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
