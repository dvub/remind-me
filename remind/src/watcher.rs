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

#[cfg(test)]
mod tests {
    use notify::Watcher;
    use std::{
        fs::File,
        io::Write,
        path::Path,
        sync::{Arc, Mutex},
        thread::{self, sleep},
        time::Duration,
    };
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
                let path_str = "test.txt";
                // create the file or rewrite it, doesn't really matter
                let mut test_file = File::create(path_str).unwrap();
                let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
                debouncer
                    .watcher()
                    .watch(Path::new(path_str), notify::RecursiveMode::NonRecursive)
                    .unwrap();
                // in an alternate, simultaneous thread,
                // write to the file to trigger the watcher
                thread::spawn(move || {
                    sleep(Duration::from_millis(100));
                    test_file.write_all(b"hello, world!").unwrap();
                    println!("Successfully wrote some data to the test file...");
                });
                println!("Waiting for file changes...");
                // wait until a message is sent

                // TODO:
                // find the correct/optimal time to wait.
                if (timeout(Duration::from_secs(5), rx.recv()).await).is_err() {
                    panic!("Receiver timed out, failing test...");
                }
                println!("Detected file changes, good.");
            });
    }
    #[test]
    fn detect_single_change() {
        // set up the file
        let path_str = "debounce.txt";
        let mut test_file = File::create(path_str).unwrap();
        // create our watcher
        let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
        debouncer
            .watcher()
            .watch(Path::new(path_str), notify::RecursiveMode::NonRecursive)
            .unwrap();
        let times_written = Arc::new(Mutex::new(0));
        // 3 * 100ms should run within one tick/frame of our debouncer,
        // thus our debouncer SHOULD only notice 1 change.
        let write_thread_handle = thread::spawn(move || {
            for _ in 0..3 {
                sleep(Duration::from_millis(100));
                test_file.write_all(b"hello, world!\n").unwrap();
            }
        });

        // we need to detect changes here while the other thread is writing file changes
        let clone = Arc::clone(&times_written);
        while !write_thread_handle.is_finished() {
            let _ = rx.blocking_recv().unwrap();
            let mut inner = clone.lock().unwrap();
            *inner += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        assert_eq!(*times_written.lock().unwrap(), 1);
    }
    // TODO: FIX
    #[test]
    fn detect_multiple_changes() {
        // set up the file
        let path_str = "debounce.txt";
        let mut test_file = File::create(path_str).unwrap();
        // create our watcher
        let (mut debouncer, mut rx) = super::gen_watcher_receiver().unwrap();
        debouncer
            .watcher()
            .watch(Path::new(path_str), notify::RecursiveMode::NonRecursive)
            .unwrap();
        let times_written = Arc::new(Mutex::new(0));
        // 3 * 100ms should run within one tick/frame of our debouncer,
        // thus our debouncer SHOULD only notice 1 change.
        let write_thread_handle = thread::spawn(move || {
            for _ in 0..3 {
                for _ in 0..3 {
                    sleep(Duration::from_millis(100));
                    test_file.write_all(b"hello, world!\n").unwrap();
                    println!("wrote changes");
                }
                sleep(Duration::from_secs(2));
                println!("inner loop finished");
            }
        });

        // we need to detect changes here while the other thread is writing file changes
        let clone = Arc::clone(&times_written);
        while !write_thread_handle.is_finished() {
            let _ = rx.blocking_recv().unwrap();
            let mut inner = clone.lock().unwrap();
            *inner += 1;
        }
        // wait for the writing thread to finish
        write_thread_handle.join().unwrap();
        assert_eq!(*times_written.lock().unwrap(), 3);
    }
}
