use clap::Parser;
use codecrafters_grep::{match_pattern, Args, Result};
use std::io;
use std::process;

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let Args { extend, pattern } = Args::parse();

    if !extend {
        eprintln!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let input = io::stdin().lock();

    if match_pattern(input, &pattern)? {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
