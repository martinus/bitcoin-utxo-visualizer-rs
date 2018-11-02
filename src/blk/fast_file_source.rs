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

/// Implements a fast file as data source for the parser. Currently this uses a
/// memory mapped file, as this seems to be the fastest solution.
pub struct FastFileSource {
    mmap: Mmap,
}

impl FastFileSource {
    /// Creates a new file source for the given filename.
    pub fn new(filename: &str) -> std::io::Result<FastFileSource> {
        let file = File::open(filename)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let ffs = FastFileSource { mmap: mmap };

        Ok(ffs)
    }

    /// Gets an iterator to the file's data, which is the input for parser::parse().
    pub fn iter(&mut self) -> std::slice::Iter<'_, u8> {
        self.mmap.as_ref().iter()
    }
}
