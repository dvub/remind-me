mod daemon;
mod task;
mod watcher;

use std::env;

use daemon::{configure_daemon, setup_file, start_daemon};

// TODO: fix error propagation/handling in general
// its a shitshow right now
pub fn main() -> anyhow::Result<()> {
    let path = env::current_dir()?.join("Config.toml");
    setup_file(&path)?;
    let daemon = configure_daemon(&env::current_dir()?)?;
    start_daemon(daemon, &path)?;
    // run(&file)?;
    Ok(())
}
