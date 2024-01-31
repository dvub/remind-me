use core::is_daemon_running;

use clap::Parser;

mod args;
use args::Args;
fn main() {
    let args = Args::parse();

    println!("Hello, world!");

    let r = is_daemon_running("remind");

    println!("is the daemon running? {r}");
}
