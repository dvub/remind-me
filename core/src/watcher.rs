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

    // TODO:
    // find correct timeout
    // maybe set a better tickrate lol
    let debouncer = new_debouncer(
        Duration::from_secs(1),
        Some(Duration::from_millis(500)),
        handler_closure,
    )?;
    Ok((debouncer, receiver))
}
// #[cfg(test)]
mod tests {
    use notify::Watcher;
    use std::{
        fs::File,
        io::{self, Write},
        path::Path,
        thread::{self, sleep},
        time::Duration,
    };
    use tempfile::tempdir;

    fn count_changes<F>(write_logic: F) -> i32
    where
        F: Fn(&mut File) -> io::Result<()> + Send + 'static,
    {
        // set up the file
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");
        let mut test_file = File::create(&path).unwrap();
        // create our watcher
        let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
        debouncer
            .watcher()
            .watch(Path::new(&path), notify::RecursiveMode::NonRecursive)
            .unwrap();
        let mut times_written = 0;

        let write_thread_handle = thread::spawn(move || {
            write_logic(&mut test_file).unwrap();
            drop(test_file);
            debouncer.stop();
        });
        // we need to detect changes here while the other thread is writing file changes
        while rx.blocking_recv().is_some() {
            times_written += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        temp_dir.close().unwrap();
        times_written
    }
    #[test]
    fn detect_single_change() {
        let changes = count_changes(|file| {
            for _ in 0..3 {
                file.write_all(b"hello, world!\n")?;
                std::thread::sleep(Duration::from_millis(10));
            }
            sleep(Duration::from_secs(2));
            Ok(())
        });
        assert_eq!(changes, 1);
    }

    #[test]
    fn detect_multiple_changes() {
        let expected_num_changes = 3;
        let changes = count_changes(move |file| {
            for _ in 0..expected_num_changes {
                file.write_all(b"hello, world!\n")?;
                std::thread::sleep(Duration::from_secs(2));
            }
            Ok(())
        });
        assert_eq!(changes, expected_num_changes);
    }

    #[test]
    fn ignore_extra_changes() {
        let outer_iter_count = 3;
        let changes = count_changes(move |file| {
            for _ in 0..outer_iter_count {
                for _ in 0..3 {
                    file.write_all(b"hello, world\n").unwrap();
                }
                sleep(Duration::from_secs(2));
            }
            Ok(())
        });
        assert_eq!(changes, outer_iter_count);
    }
}
