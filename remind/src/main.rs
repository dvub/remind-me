mod daemon;
mod task;
mod watcher;

use std::env;

use daemon::{configure_daemon, run};
use watcher::setup_config;
// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

fn main() -> anyhow::Result<()> {
    let file = setup_config()?;
    let _daemon = configure_daemon(&env::current_dir()?)?;
    //start_daemon(daemon, &file)?;
    run(&file)?;
    Ok(())
}
