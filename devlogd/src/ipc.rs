use std::path::{Path, PathBuf};

use sqlx::SqlitePool;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;

use crate::watchers::git::poll_git;

pub async fn start_socket_server(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;

    let socket_dir = PathBuf::from(home).join(".devlog");

    std::fs::create_dir_all(&socket_dir)?;

    let socket_path = socket_dir.join("devlogd.sock");

    println!("socket listening at {}", socket_path.display());

    let _ = std::fs::remove_file(&socket_path);

    let listener = UnixListener::bind(socket_path)?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let pool = pool.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let msg = String::from_utf8_lossy(&buf[..n]);

            println!("{}", msg);

            // Parse: "commit|/path/to/project"
            if let Some(path) = msg.strip_prefix("commit|") {
                let project_path = path.trim();
                if let Err(e) = poll_git(project_path, &pool).await {
                    eprintln!("Error polling git: {}", e);
                }
            }
        });
    }
}
