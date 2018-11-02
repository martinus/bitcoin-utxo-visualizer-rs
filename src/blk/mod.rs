pub use self::fast_file_source::FastFileSource;
mod fast_file_source;

pub use self::parser::parse;
pub use self::parser::BlockCallback;
mod parser;