use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'E')]
    pub extend: bool,
    pub pattern: String,
}
