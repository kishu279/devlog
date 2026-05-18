use devlog_core::{get_cmds_today, get_commits_today, get_files_today, SqlitePool};
use std::collections::HashMap;
use std::fmt::Write;

pub async fn build(pool: &SqlitePool) -> String {
    let mut out = String::new();

    match get_commits_today(pool).await {
        Ok(commits) => {
            let _ = writeln!(out, "COMMITS TODAY ({}):", commits.len());
            for e in &commits {
                let p: serde_json::Value = serde_json::from_str(&e.payload).unwrap_or_default();
                let commit = payload_section(&p, "Commit");
                let files = commit["files_changed"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|v| v.as_str().unwrap_or(""))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();

                let _ = writeln!(
                    out,
                    "  {}  {}\n     files: {}  +{} -{}",
                    &e.ts,
                    commit["message"].as_str().unwrap_or(""),
                    files,
                    commit["insertions"].as_i64().unwrap_or(0),
                    commit["deletions"].as_i64().unwrap_or(0),
                );
            }
        }
        Err(error) => {
            let _ = writeln!(out, "COMMITS TODAY: query failed: {error}");
        }
    }

    match get_files_today(pool).await {
        Ok(files) => {
            let mut file_counts: HashMap<String, u32> = HashMap::new();
            for e in &files {
                let p: serde_json::Value = serde_json::from_str(&e.payload).unwrap_or_default();
                let file_change = payload_section(&p, "FileChange");
                if let Some(path) = file_change["path"].as_str().or_else(|| p["path"].as_str()) {
                    *file_counts.entry(path.to_string()).or_insert(0) += 1;
                }
            }
            let _ = writeln!(out, "\nFILES TOUCHED ({}):", file_counts.len());
            let mut file_list: Vec<_> = file_counts.iter().collect();
            file_list.sort_by(|a, b| b.1.cmp(a.1));
            for (path, count) in file_list.iter().take(8) {
                let _ = writeln!(out, "  {}  ({} saves)", path, count);
            }
        }
        Err(error) => {
            let _ = writeln!(out, "\nFILES TOUCHED: query failed: {error}");
        }
    }

    match get_cmds_today(pool).await {
        Ok(commands) => {
            let _ = writeln!(out, "\nSHELL COMMANDS TODAY ({}):", commands.len());
            for e in commands.iter().take(8) {
                let p: serde_json::Value = serde_json::from_str(&e.payload).unwrap_or_default();
                let shell = payload_section(&p, "ShellCommand");
                let command = shell["command"]
                    .as_str()
                    .or_else(|| shell["cmd"].as_str())
                    .or_else(|| shell["line"].as_str())
                    .unwrap_or_else(|| e.payload.as_str());
                let _ = writeln!(out, "  {}  {}", e.ts, truncate(command, 160));
            }
        }
        Err(error) => {
            let _ = writeln!(out, "\nSHELL COMMANDS TODAY: query failed: {error}");
        }
    }

    out
}

pub async fn build_diagnostics(pool: &SqlitePool) -> String {
    let mut out = String::new();
    let db_path = devlog_core::db_path();
    let midnight = devlog_core::today_midnight_ts();

    let _ = writeln!(out, "DB path: {}", db_path.display());
    let _ = writeln!(out, "Today cutoff ts: {midnight}");
    let _ = writeln!(out);
    let _ = writeln!(out, "TABLE ROW COUNTS:");

    let tables = match sqlx::query_scalar::<_, String>(
        r#"
        SELECT name
        FROM sqlite_master
        WHERE type = 'table'
          AND name NOT LIKE 'sqlite_%'
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
    {
        Ok(tables) => tables,
        Err(error) => {
            let _ = writeln!(out, "  failed to read sqlite_master: {error}");
            return out;
        }
    };

    if tables.is_empty() {
        let _ = writeln!(out, "  no tables found");
    }

    for table in &tables {
        let sql = format!("SELECT COUNT(*) FROM {}", quote_identifier(table));
        match sqlx::query_scalar::<_, i64>(&sql).fetch_one(pool).await {
            Ok(count) => {
                let _ = writeln!(out, "  {table}: {count} rows");
            }
            Err(error) => {
                let _ = writeln!(out, "  {table}: count failed: {error}");
            }
        }
    }

    if !tables.iter().any(|table| table == "events") {
        let _ = writeln!(
            out,
            "\nEVENTS TABLE: missing, so standup cannot read activity yet"
        );
        return out;
    }

    let _ = writeln!(out, "\nEVENT KINDS:");
    match sqlx::query_as::<_, KindCount>(
        r#"
        SELECT kind, COUNT(*) AS row_count
        FROM events
        GROUP BY kind
        ORDER BY row_count DESC, kind ASC
        "#,
    )
    .fetch_all(pool)
    .await
    {
        Ok(kind_counts) if kind_counts.is_empty() => {
            let _ = writeln!(out, "  none");
        }
        Ok(kind_counts) => {
            for row in kind_counts {
                let _ = writeln!(out, "  {}: {} rows", row.kind, row.row_count);
            }
        }
        Err(error) => {
            let _ = writeln!(out, "  failed: {error}");
        }
    }

    let _ = writeln!(out, "\nCONTEXT QUERIES:");
    append_bucket_count(
        pool,
        &mut out,
        "commits",
        "events WHERE kind IN ('Commit', 'commit') AND ts >= today_cutoff",
        "SELECT COUNT(*) FROM events WHERE kind IN ('Commit', 'commit') AND ts >= ?",
        midnight,
    )
    .await;
    append_bucket_count(
        pool,
        &mut out,
        "file changes",
        "events WHERE kind IN ('FileChange', 'file_change', 'file') AND ts >= today_cutoff",
        "SELECT COUNT(*) FROM events WHERE kind IN ('FileChange', 'file_change', 'file') AND ts >= ?",
        midnight,
    )
    .await;
    append_bucket_count(
        pool,
        &mut out,
        "shell commands",
        "events WHERE kind IN ('ShellCommand', 'shell_command', 'cmd') AND ts >= today_cutoff",
        "SELECT COUNT(*) FROM events WHERE kind IN ('ShellCommand', 'shell_command', 'cmd') AND ts >= ?",
        midnight,
    )
    .await;

    let _ = writeln!(out, "\nRECENT EVENTS:");
    match sqlx::query_as::<_, EventPreview>(
        r#"
        SELECT id, ts, kind, project, payload
        FROM events
        ORDER BY ts DESC, id DESC
        LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await
    {
        Ok(events) if events.is_empty() => {
            let _ = writeln!(out, "  none");
        }
        Ok(events) => {
            for event in events {
                let _ = writeln!(
                    out,
                    "  #{id} {ts} {kind} {project} payload={payload}",
                    id = event.id,
                    ts = event.ts,
                    kind = event.kind,
                    project = truncate(&event.project, 60),
                    payload = truncate(&event.payload, 140)
                );
            }
        }
        Err(error) => {
            let _ = writeln!(out, "  failed: {error}");
        }
    }

    out
}

async fn append_bucket_count(
    pool: &SqlitePool,
    out: &mut String,
    label: &str,
    description: &str,
    sql: &str,
    midnight: i64,
) {
    match sqlx::query_scalar::<_, i64>(sql)
        .bind(midnight)
        .fetch_one(pool)
        .await
    {
        Ok(count) => {
            let _ = writeln!(out, "  {label}: {count} rows from {description}");
        }
        Err(error) => {
            let _ = writeln!(out, "  {label}: failed from {description}: {error}");
        }
    }
}

fn payload_section<'a>(payload: &'a serde_json::Value, section: &str) -> &'a serde_json::Value {
    payload.get(section).unwrap_or(payload)
}

fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}

#[derive(sqlx::FromRow)]
struct KindCount {
    kind: String,
    row_count: i64,
}

#[derive(sqlx::FromRow)]
struct EventPreview {
    id: i64,
    ts: i64,
    kind: String,
    project: String,
    payload: String,
}
