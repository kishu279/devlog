use crate::watchers::git::*;

mod event;
mod ipc;
mod store;
mod watchers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("devlogd daemon starting...");
    // checks which os is this

    Ok(())
}
