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
// TODO: reduce repetition
#[cfg(test)]
mod tests {
    use notify::Watcher;
    use std::{
        fs::File,
        io::Write,
        path::Path,
        sync::{Arc, Mutex},
        thread::{self, sleep, JoinHandle},
        time::Duration,
    };
    use tempfile::tempdir;
    use tokio::time::timeout;

    // TODO:
    // mark with #[tokio::test]
    // at the time of me writing this, #[tokio::test] was not compiling for whatever reason
    // so i just built a runtime and blocked on it
    #[test]
    fn test_watcher() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let temp_dir = tempdir().unwrap();
                let path = temp_dir.path().join("test.txt");
                let mut test_file = File::create(&path).unwrap();
                // println!("{:?}", test_file.path().display());
                let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
                debouncer
                    .watcher()
                    .watch(&path, notify::RecursiveMode::NonRecursive)
                    .unwrap();
                // in an alternate, simultaneous thread,
                // write to the file to trigger the watcher
                thread::spawn(move || {
                    sleep(Duration::from_millis(100));
                    test_file.write_all(b"hello, world!").unwrap();
                    println!("successfully wrote");
                    drop(test_file);
                });
                // TODO:
                // find the correct/optimal time to wait.
                if (timeout(Duration::from_secs(5), rx.recv()).await).is_err() {
                    panic!("Receiver timed out, failing test...");
                }

                temp_dir.close().unwrap();
            });
    }

    #[test]
    fn detect_single_change() {
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
            for _ in 0..3 {
                test_file.write_all(b"hello, world!\n").unwrap();
                sleep(Duration::from_millis(100));
            }
            // TODO: fix this extra delay FFS!
            sleep(Duration::from_secs(2));
            debouncer.stop();
        });
        // we need to detect changes here while the other thread is writing file changes
        while rx.blocking_recv().is_some() {
            times_written += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        temp_dir.close().unwrap();
        assert_eq!(times_written, 1);
    }

    #[test]
    fn detect_multiple_changes() {
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
            for _ in 0..3 {
                test_file.write_all(b"hello, world!\n").unwrap();
                sleep(Duration::from_secs(2));
            }
            debouncer.stop();
        });
        // we need to detect changes here while the other thread is writing file changes
        while rx.blocking_recv().is_some() {
            times_written += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        temp_dir.close().unwrap();
        assert_eq!(times_written, 3);
    }

    #[test]
    fn ignore_extra_changes() {
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
            for _ in 0..3 {
                for _ in 0..3 {
                    test_file.write_all(b"hello, world\n").unwrap();
                }
                sleep(Duration::from_secs(2));
            }
            debouncer.stop();
        });
        // we need to detect changes here while the other thread is writing file changes
        while rx.blocking_recv().is_some() {
            times_written += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        temp_dir.close().unwrap();
        assert_eq!(times_written, 3);
    }
}
