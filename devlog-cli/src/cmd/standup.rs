use crate::{context, llm};
use devlog_core::{get_cmds_today, get_commits_today, get_files_today};

pub async fn run_standup() -> Result<(), Box<dyn std::error::Error>> {
    let pool = devlog_core::get_pool().await.expect("cannot open DB");

    let commits = get_commits_today(&pool).await.map(|v| v.len()).unwrap_or(0);
    let files = get_files_today(&pool).await.map(|v| v.len()).unwrap_or(0);
    let cmds = get_cmds_today(&pool).await.map(|v| v.len()).unwrap_or(0);

    println!("commits: {}  files: {}  commands: {}", commits, files, cmds);

    if commits == 0 && files == 0 && cmds == 0 {
        println!("\nNo activity recorded today. Make sure devlogd is running.");
        return Ok(());
    }

    println!("\nGenerating standup...");

    let ctx = context::build(&pool).await;
    let standup = llm::call_claude(&ctx).await;
    println!("\n{}", standup);

    Ok(())
}
