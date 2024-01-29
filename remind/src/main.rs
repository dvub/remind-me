use core::{configure_daemon, setup_config, start_daemon};
use std::env;

// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

fn main() -> anyhow::Result<()> {
    let file = setup_config()?;
    let daemon = configure_daemon(&env::current_dir()?)?;
    start_daemon(daemon, &file)?;
    Ok(())
}
