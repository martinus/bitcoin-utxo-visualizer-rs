// Copyright 2016 Martin Ankerl.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::fast_file_source::FastFileSource;
mod fast_file_source;

pub use self::parser::parse;
pub use self::parser::BlockCallback;
mod parser;

mod change_to_pixel;