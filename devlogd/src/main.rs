use crate::watchers::git::*;

mod event;
mod ipc;
mod store;
mod watchers;
mod helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("devlogd daemon starting...");
    // checks which os is this

    let project = "/home/kishu/Desktop/hackathon/ibm-bob-hackathon/devlog/demo";
    poll_git(project).await?;

    Ok(())
}
