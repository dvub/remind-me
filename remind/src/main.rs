mod daemon;
mod task;
mod watcher;

use std::env;

use daemon::{configure_daemon, setup_config, start_daemon};

// TODO: fix error propagation/handling in general
// its a shitshow right now
fn main() -> anyhow::Result<()> {
    let file = setup_config()?;
    let daemon = configure_daemon(&env::current_dir()?)?;
    start_daemon(daemon, &file)?;
    // run(&file)?;
    Ok(())
}
