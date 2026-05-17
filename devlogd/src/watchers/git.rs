use tokio::process::Command;

use crate::{
    event::{
        event::DevlogEvent,
        kind::EventKind,
        payload::{commit::CommitPayload, EventPayload},
    },
    helper::convert_datetime_to_i64,
    store::events::insert_event,
};

use sqlx::SqlitePool;

pub async fn poll_git(project: &str, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // get the specific items from the logs
    // hash
    // commit message
    // timestamps

    let output = Command::new("git")
        .args([
            "--no-pager",
            "log",
            "--since=midnight",
            "--format=%H|%s|%ai",
        ])
        .current_dir(project)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("git log failed in {project}: {}", stderr.trim()),
        )
        .into());
    }

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(3, '|').collect();

        if parts.len() != 3 {
            eprintln!("invalid git log line: {}", line);
            continue;
        }

        let hash = parts[0];
        let commit_message = parts[1];
        let time_stamp = convert_datetime_to_i64(parts[2]);
        // get the files changed and store
        // files changed and insertion and deletion

        let output = Command::new("git")
            .args(["--no-pager", "show", hash, "--numstat", "--format="])
            .current_dir(project)
            .output()
            .await
            .expect("failed to execute process")
            .stdout;

        let line = String::from_utf8_lossy(&output);
        // println!("{}", line);

        let mut total_insertions = 0;
        let mut total_deletions = 0;
        let mut file_changed: Vec<String> = Vec::new();

        for line in line.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // error
            if parts.len() < 3 {
                continue;
            }

            let insertions = parts[0].parse::<i64>().unwrap_or(0);
            let deletions = parts[1].parse::<i64>().unwrap_or(0);

            // filepath may contain spaces
            let file = parts[2..].join(" ");

            total_insertions += insertions;
            total_deletions += deletions;

            file_changed.push(file);
        }

        // get the branch details
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(project)
            .output()
            .await
            .expect("failed to execute process")
            .stdout;

        let branch = String::from_utf8_lossy(&output).trim().to_string();

        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(project)
            .output()
            .await
            .expect("failed to execute process")
            .stdout;

        let project = String::from_utf8_lossy(&output).trim().to_string();

        let log_data = DevlogEvent {
            kind: EventKind::Commit,
            ts: time_stamp,
            project,
            payload: EventPayload::Commit(CommitPayload {
                branch: branch.to_string(),
                deletions: total_deletions,
                files_changed: file_changed,
                insertions: total_insertions,
                hash: hash.to_string(),
                message: commit_message.to_string(),
            }),
        };

        let json = serde_json::to_string_pretty(&log_data).unwrap();

        println!("{}", json);

        // making the call to the database
        insert_event(&log_data, pool).await?;
    }

    Ok(())
}
