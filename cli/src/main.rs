use clap::Parser;
mod args;
use args::Args;
fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
