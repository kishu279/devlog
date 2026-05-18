use inquire::Text;
use std::io::Write;

use std::{
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn handle_setup(project: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let folder_path_input = match project {
        Some(path_str) if !path_str.is_empty() => path_str.clone(),
        _ => {
            // Falls back here if flag is missing altogether OR passed as empty string ""
            Text::new("Drag and drop your project folder here (or paste the path):")
                .with_help_message(
                    "Press Enter to use the current directory, or paste a relative/absolute path.",
                )
                .with_default(".")
                .prompt()
                .unwrap()
        }
    };

    let folder_path = resolve_input_path(&folder_path_input)?;

    if folder_path.is_dir() {
        println!("\nSuccess!");
        println!(
            "Linked Project Main Directory: {}",
            folder_path.to_string_lossy()
        );
    } else {
        return Err(format!(
            "path is not a valid folder: {}",
            folder_path.to_string_lossy()
        )
        .into());
    }

    let path_buf: Vec<PathBuf> = handle_check_dir(&folder_path)?;

    // do some operation like going to each of them and installing the hook
    // ...

    for item in path_buf {
        println!("item found git on this folder {:?}", item);

        let project_root = item.parent().unwrap();

        // install the hook function
        if let Err(e) = install_hook(&project_root) {
            eprintln!("Failed to install hook for {:?}: {}", item, e);
        } else {
            println!("Hook installed successfully for {:?}", item);
        }
    }

    Ok(())
}

fn resolve_input_path(input: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cleaned = input.trim().trim_matches(['"', '\'']);
    let path = if cleaned.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(cleaned)
    };

    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };

    Ok(path)
}

// FIND THE EACH PROJECT RECURSIVELY AND FIND THE .GIT
fn handle_check_dir(path: &PathBuf) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut path_buf: Vec<PathBuf> = Vec::new();

    let mut walker = WalkDir::new(path)
        .follow_links(true)
        .max_depth(6)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();

            !matches!(name.as_ref(), "node_modules" | "target" | ".cache")
        });

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Result::Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() && path.ends_with(".git") {
            println!("Found the git: {:?}", path);
            let absolute = std::fs::canonicalize(path)?;
            println!("{:?}", absolute);
            path_buf.push(absolute);
            walker.skip_current_dir(); // Tells WalkDir not to look inside this .git folder
            continue; // skip finding the project inside that project we are inside
        }
    }

    Ok(path_buf)
}

// installing hook on the .git/hook
fn install_hook(repo: &Path) -> Result<(), anyhow::Error> {
    let hook_path = repo.join(".git/hooks/post-commit");
    let hook_line =
        r#"printf 'commit|%s' "$(pwd)" | nc -U ~/.devlog/devlogd.sock 2>/dev/null || true"#;
    if hook_path.exists() {
        // Check if already installed (idempotent)
        let content = fs::read_to_string(&hook_path)?;
        if content.contains("devlogd.sock") {
            return Ok(()); // Already there, skip
        }
        // Append to existing hook
        let mut file = fs::OpenOptions::new().append(true).open(&hook_path)?;
        writeln!(file, "\n# devlog\n{}", hook_line)?;
    } else {
        // Create new hook
        fs::write(&hook_path, format!("#!/bin/sh\n{}\n", hook_line))?;
        fs::set_permissions(&hook_path, Permissions::from_mode(0o755))?;
    }
    Ok(())
}
