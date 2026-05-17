use crate::{store::db::connect_db, watchers::git::*};

mod event;
mod helper;
mod ipc;
mod store;
mod watchers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("devlogd daemon starting...");
    // checks which os is this

    let pool = connect_db().await?;

    let project = "/home/kishu/Desktop/hackathon/ibm-bob-hackathon/devlog/demo";

    // event loop every 60s
    poll_git(project, &pool).await?;

    Ok(())
}
