use std::path::PathBuf;

// // to be implemented
use notify::{Event, RecursiveMode, Result, Watcher};
use tokio::sync::mpsc::Sender;

// SETUP WATCHER
pub fn file_watcher_initialize(
    path: &PathBuf,
    tx: Sender<notify::Result<notify::Event>>,
) -> Result<()> {
    let mut watcher = notify::recommended_watcher(move |event| {
        let _ = tx.blocking_send(event);
    })?;

    // recursive working
    watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive)?;

    Ok(())
}
