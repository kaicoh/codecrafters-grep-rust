mod args;
mod error;

use std::io::BufRead;

pub use args::Args;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn match_pattern<R: BufRead>(buf: R, pattern: &str) -> Result<bool> {
    for line in buf.lines() {
        let line = line?;

        if search(&line, pattern) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn search(line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => line.chars().any(|c| c.is_ascii_digit()),
        _ => line.contains(pattern),
    }
}
