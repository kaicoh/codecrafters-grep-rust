mod args;
mod error;
mod re;

use re::Regex;
use std::io::BufRead;

pub use args::Args;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn match_pattern<R: BufRead>(buf: R, pattern: &str) -> Result<bool> {
    let regex = Regex::new(pattern);

    for line in buf.lines() {
        let line = line?;

        if regex.is_match(&line) {
            return Ok(true);
        }
    }

    Ok(false)
}
