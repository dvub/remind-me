use daemonize::Daemonize;
use std::fs::File;
use std::path::Path;

// TODO:
// testing
// unify project dir instead of calling it in individual files

pub mod control {
    use crate::{get_dir, get_path, run};

    use super::configure_daemon;
    use std::{fs::File, io::Read, str::FromStr};
    use sysinfo::{Pid, System};

    /// Takes a `Daemonize` and a target config file. Runs the program as a daemon
    /// reading reminders from the target config file.
    pub fn start_daemon() -> anyhow::Result<()> {
        let dir = get_dir()?;
        let path = get_path()?;
        let daemon = configure_daemon(&dir)?;
        match daemon.start() {
            Ok(_) => {
                run(path)?;
            }
            Err(e) => eprintln!("there was an error starting the daemon: {e}"),
        }
        Ok(())
    }
    // TODO:
    // handle when pid doesn't exist
    // big issue for first time running
    fn get_pid() -> anyhow::Result<Pid> {
        let dir = get_dir()?;
        // println!("{:?}", dir.display());
        let path = dir.join("remind.pid");

        let mut file = File::open(path)?;
        // TODO:
        // check if file even exists
        let mut str = String::new();
        file.read_to_string(&mut str)?;
        let trimmed = str.trim();
        Ok(Pid::from_str(trimmed)?)
    }

    /*
        pub fn get_daemon_stats() -> anyhow::Result<()> {
            let pid = get_pid()?;
            let system = System::new_all();

            let process = system.process(pid).unwrap();

            let mem = process.virtual_memory();
            let cpu = process.cpu_usage();
            let x = process.disk_usage().;
            Ok(())
        }
    */
    // TODO:
    // maybe there's a better way to determine if daemon is running?
    // fix error handlling FFS
    pub fn is_daemon_running() -> anyhow::Result<bool> {
        let pid = get_pid()?;
        let system = System::new_all();
        let is_running = system.process(pid).is_some();
        Ok(is_running)

        // system.process(Pid::from_u32(u)).is_some()
    }
    pub fn stop_daemon() -> anyhow::Result<()> {
        let pid = get_pid()?;

        let system = System::new_all();
        if let Some(process) = system.process(pid) {
            process.kill();
            // println!("Stopped the daemon");
        }
        Ok(())
    }

    // TODO: implement stop
}

/// Configure and return a `Daemonize<()>`.
fn configure_daemon(dir: &Path) -> anyhow::Result<Daemonize<()>> {
    println!("configuring daemon...");
    let stdout = File::create(dir.join("daemon.out"))?;
    let stderr = File::create(dir.join("daemon.err"))?;
    let daemonize = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .pid_file(dir.join("remind.pid"));
    // .working_directory(dir);
    Ok(daemonize)
}
