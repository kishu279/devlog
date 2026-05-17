use inquire::Text;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn handle_setup(project: Option<String>) {
    // let log_title = Text::new("What is the title of this project log?")
    //     .with_placeholder("e.g., Project Alpha")
    //     .prompt()
    //     .unwrap_or_else(|_| "Untitled".to_string());

    let folder_path_input = match project {
        Some(path_str) if !path_str.is_empty() => path_str.clone(),
        _ => {
            // Falls back here if flag is missing altogether OR passed as empty string ""
            Text::new("Drag and drop your project folder here (or paste the path):")
                .with_help_message(
                    "You can physically drag a folder from your desktop into this window.",
                )
                .prompt()
                .unwrap()
        }
    };

    // 3. Clean up shell quotes and whitespace from drag-and-drop actions
    let folder_path = PathBuf::from(folder_path_input.trim().trim_matches(['"', '\'']));

    // 4. Validate the directory
    if folder_path.is_dir() {
        println!("\n✅ Success!");
        // println!("Project Log:   {}", log_title);
        println!("Linked Folder: {}", folder_path.display());
    } else {
        println!("\n❌ Error: The path provided is not a valid folder!");
    }

    handle_check_dir(folder_path);
}

// FIND THE EACH PROJECT RECURSIVELY AND FIND THE .GIT
fn handle_check_dir(path: PathBuf) {
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
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() && path.ends_with(".git") {
            println!("Found the git: {:?}", path);
            walker.skip_current_dir(); // Tells WalkDir not to look inside this .git folder
            continue; // skip finding the project inside that project we are inside 
        }
    }
}
