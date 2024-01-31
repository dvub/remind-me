mod daemon;
mod task;
mod watcher;

use std::env;

use daemon::{configure_daemon, run, setup_config};
fn main() -> anyhow::Result<()> {
    let file = setup_config()?;
    let _daemon = configure_daemon(&env::current_dir()?)?;
    //start_daemon(daemon, &file)?;
    run(&file)?;
    Ok(())
}
