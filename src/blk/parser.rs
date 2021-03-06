// Copyright 2016 Martin Ankerl.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This callback is called while parsing the blk data file.
pub trait BlockCallback {
    /// Called whenever a new block starts.
    /// * `block_height`: The block height, starting by 0, of the current block.
    fn begin_block(&mut self, block_height: u32);

    /// Called for each UTXO that is created/destroyed within the block started with `begin_block`.
    /// * `block_height`: Block height of the UTXO that is created/destroyed.
    /// * `amount_satoshi`: changed amount, in satoshi. Positive if UTXO is created, negative if UTXO is destroyed.
    fn change(&mut self, block_height: u32, amount_satoshi: i64);

    /// Called whenever a block has finished processing.
    /// * `block_height`: Block height of the ending block. Same as when previously `begin_block` was called.
    fn end_block(&mut self, block_height: u32);
}

fn next<T: From<u8>>(it: &mut std::slice::Iter<u8>) -> Option<T> {
    return Some((*it.next()?).into());
}

fn parse_u32(it: &mut std::slice::Iter<u8>) -> Option<u32> {
    let mut x: u32 = 0;
    x |= next::<u32>(it)? << (8 * 0);
    x |= next::<u32>(it)? << (8 * 1);
    x |= next::<u32>(it)? << (8 * 2);
    x |= next::<u32>(it)? << (8 * 3);
    return Some(x);
}

fn parse_u64(it: &mut std::slice::Iter<u8>) -> Option<u64> {
    let mut x: u64 = 0;
    x |= next::<u64>(it)? << (8 * 0);
    x |= next::<u64>(it)? << (8 * 1);
    x |= next::<u64>(it)? << (8 * 2);
    x |= next::<u64>(it)? << (8 * 3);

    x |= next::<u64>(it)? << (8 * 4);
    x |= next::<u64>(it)? << (8 * 5);
    x |= next::<u64>(it)? << (8 * 6);
    x |= next::<u64>(it)? << (8 * 7);
    return Some(x);
}

fn parse_var_u64(it: &mut std::slice::Iter<u8>) -> Option<(u32, u64)> {
    let mut val: u64 = 0;
    let mut num_bytes = 0;
    loop {
        let byte = *it.next()?;
        val |= (byte as u64) << (7 * num_bytes);
        num_bytes += 1;
        if byte <= 0b01111111 {
            return Some((num_bytes, val));
        }
    }
}

fn parse_var_i32(it: &mut std::slice::Iter<u8>) -> Option<(u32, i32)> {
    let (bytes, val) = parse_var_u64(it)?;
    let val = (val >> 1) ^ 0_u64.wrapping_sub(val & 1);
    return Some((bytes, val as i32));
}

fn parse_i64(it: &mut std::slice::Iter<u8>) -> Option<i64> {
    return Some(parse_u64(it)? as i64);
}

pub fn parse(it: &mut std::slice::Iter<u8>, callback: &mut BlockCallback) -> Option<()> {
    loop {
        match parse_u32(it) {
            None => {
                return Some(());
            }
            Some(magick_blk0) => {
                if 0x004b4c42 != magick_blk0 {
                    return None;
                }
            }
        }

        let current_block_height = parse_u32(it)?;
        callback.begin_block(current_block_height);
        let num_bytes_total = parse_u32(it)?;

        let mut amount = parse_i64(it)?;
        let mut amount_block_height = parse_u32(it)?;
        callback.change(amount_block_height, amount);

        let mut bytes_read = 4 + 8;
        while bytes_read < num_bytes_total {
            let (bytes, amount_diff) = parse_var_u64(it)?;
            bytes_read += bytes;
            amount += amount_diff as i64;

            let (bytes, block_diff) = parse_var_i32(it)?;
            bytes_read += bytes;
            amount_block_height = amount_block_height.wrapping_add(block_diff as u32);
            callback.change(amount_block_height, amount);
        }
    }
}
