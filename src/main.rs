// Copyright 2016 Martin Ankerl.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

pub mod blk;

/// Checks correct order of blocks in the file.
pub struct CheckSequential {
    last_block: u32,
    num_changes: u32,
}

impl CheckSequential {
    fn new() -> CheckSequential {
        return CheckSequential {
            last_block: 0_u32.wrapping_sub(1),
            num_changes: 0,
        };
    }

    fn print(&self) {
        println!(
            "last block: {}, num changes: {}",
            self.last_block, self.num_changes
        );
    }
}

impl blk::BlockCallback for CheckSequential {
    fn begin_block(&mut self, block_height: u32) {
        if block_height != self.last_block.wrapping_add(1) {
            println!(
                "begin_block: expected block {} but got {}",
                self.last_block + 1,
                block_height
            );
        }
        self.last_block = block_height;
    }

    fn change(&mut self, _block_height: u32, _amount: i64, _is_same_as_previous_change: bool) {
        self.num_changes += 1;
    }
    fn end_block(&mut self, block_height: u32) {
        if block_height != self.last_block {
            println!(
                "begin_block: expected block {} but got {}",
                self.last_block, block_height
            );
        }
    }
}

fn main() -> std::io::Result<()> {
    // get filename as first argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // process each byte
    let mut callback = CheckSequential::new();

    let mut ffs = blk::FastFileSource::new(filename)?;

    let mut iter = ffs.iter();
    match blk::parse(&mut iter, &mut callback) {
        None => {
            println!("Something bad happened");
        }
        Some(()) => {
            println!("parsing successfull!");
        }
    }

    callback.print();
    Ok(())
}
