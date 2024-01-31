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
    let handler_closure = move |result: Result<Vec<DebouncedEvent>, _>| match result {
        Ok(debounced_events) => {
            for event in debounced_events {
                if let EventKind::Modify(_) = event.kind {
                    tx.blocking_send(event).unwrap();
                }
            }
        }
        Err(e) => {
            println!("there was an error reading debounced changes: {e:?}")
        }
    };
    // note that the debouncer must be returned
    // so that it's not dropped (and stops sending to the receiver)
    let debouncer = new_debouncer(Duration::from_secs(1), None, handler_closure)?;
    Ok((debouncer, receiver))
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::Write,
        path::Path,
        thread::{self, sleep},
        time::Duration,
    };

    use notify::Watcher;

    #[test]
    #[allow(unused_assignments)]
    fn test_watcher() {
        let path_str = "test.txt";
        // create the file or rewrite it, doesn't really matter
        let mut test_file = File::create(path_str).unwrap();
        let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
        debouncer
            .watcher()
            .watch(Path::new(path_str), notify::RecursiveMode::NonRecursive)
            .unwrap();

        let mut was_written = false;
        // in an alternate, simultaneous thread,
        // write to the file to trigger the watcher
        thread::spawn(move || {
            sleep(Duration::from_secs(1));
            test_file.write_all(b"hello, world!").unwrap();
        });
        // wait until a message is sent
        let _ = rx.blocking_recv().unwrap();
        was_written = true;
        assert!(was_written);
    }
}
