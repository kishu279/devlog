use crate::{store::db::connect_db, watchers::git::poll_git};

mod event;
mod helper;
mod ipc;
mod project;
mod store;
mod watchers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("devlogd daemon starting...");
    // checks which os is this

    // PHASE 1 DONE
    let pool = connect_db().await?;

    let project = "/home/kishu/Desktop/hackathon/ibm-bob-hackathon/devlog/demo";

    // event loop every 60s
    poll_git(project, &pool).await?;

    // PHASE3 DAEMON PROCESS TO RUN THE FILE WATCHER
    // // async catcher or listener
    // let (tx, mut rx) = mpsc::channel::<Result<Event>>(32);

    // let mut watcher = notify::recommended_watcher(move |event| {
    //     let _ = tx.blocking_send(event);
    // })?;

    // // recursive working
    // watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive)?;

    // while let Some(event_result) = rx.recv().await {
    //     match event_result {
    //         Ok(event) => {
    //             println!("file event: {:?}", event);
    //         }
    //         Err(err) => {
    //             eprintln!("watch error: {:?}", err);
    //         }
    //     }
    // }

    ////////
    ////
    // PHASE3 CREATE THE PROJECTS TABLE AND MAKE QUERY AND INSTALLT HE HOOK ON THE .GIT AND THE SOCK TO MAKE THE POLL GIT INVOKING
    // PHASE4 SHELL CHECK
    // PAHSE5 CREATE THE DAEMON SERVICE
    // PAHSE6 DONT KNOW WHAT TO DO AFTER THIS ......

    Ok(())
}
