use crate::{ipc::start_socket_server, store::db::connect_db};

mod event;
mod helper;
mod ipc;
mod project;
mod store;
mod watchers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_db().await?;

    // Spawn socket server
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        start_socket_server(pool_clone).await.unwrap();
    });

    // Keep main alive
    tokio::signal::ctrl_c().await?;
    // PHASE3 CREATE THE PROJECTS TABLE AND MAKE QUERY AND INSTALLT HE HOOK ON THE .GIT AND THE SOCK TO MAKE THE POLL GIT INVOKING
    // PHASE4 SHELL CHECK
    // PAHSE5 CREATE THE DAEMON SERVICE
    // PAHSE6 DONT KNOW WHAT TO DO AFTER THIS ......

    Ok(())
}
