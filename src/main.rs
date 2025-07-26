use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    src: PathBuf,
}

fn main() {
    let _args = Args::parse();
}
