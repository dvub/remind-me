use notify::{EventKind, INotifyWatcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver};

// taken from the notify crate example here:
// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs

// this is magical
/// Generates and returns a tuplet of a file watcher a receiver.
/// The watcher must be configured outside of this function to watch a file.
/// The receiver will receive a message anytime the target file is modified.
///
pub fn gen_watcher_receiver() -> anyhow::Result<(
    Debouncer<INotifyWatcher, FileIdMap>,
    Receiver<DebouncedEvent>,
)> {
    let (tx, receiver) = channel(1);
    let debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |result: Result<Vec<DebouncedEvent>, _>| match result {
            Ok(e) => {
                for t in e {
                    match t.kind {
                        EventKind::Modify(_) => {
                            println!("Modification occurred");
                            tx.blocking_send(t).unwrap();
                        }
                        _ => {
                            println!("Something happened that I don't care about")
                        }
                    }
                }
            }
            Err(e) => {
                println!("there was an error reading debounced changes: {e:?}")
            }
        },
    )?;

    Ok((debouncer, receiver))
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use notify::Watcher;

    #[test]
    fn test_watcher() {
        let path_str = "test.txt";
        // create the file or rewrite it, doesn't really matter
        let file = File::create(path_str).unwrap();
        let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
        debouncer
            .watcher()
            .watch(Path::new(path_str), notify::RecursiveMode::NonRecursive)
            .unwrap();
    }
}
