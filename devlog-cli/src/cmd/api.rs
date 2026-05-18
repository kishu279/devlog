use inquire::{Confirm, Password};
use serde_json::json;
use std::{fs, path::PathBuf, pin::Pin};

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("cannot find home dir")
        .join(".devlog/config")
}

pub fn load_api_key() -> Option<String> {
    if let Ok(key) =
        std::env::var("OPENROUTER_API_KEY").or_else(|_| std::env::var("OPENROUTER_API"))
    {
        return Some(key);
    }
    let content = fs::read_to_string(config_path()).ok()?;
    content
        .lines()
        .find(|l| l.starts_with("OPENROUTER_API_KEY="))
        .map(|l| l["OPENROUTER_API_KEY=".len()..].trim().to_string())
}

pub async fn handle_api(
    key: Option<String>,
    show: bool,
    clear: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = config_path();

    if show {
        match load_api_key() {
            Some(k) => println!("Current API key: {}", k),
            None => println!("No API key saved."),
        }
        return Ok(());
    }

    if clear {
        if config.exists() {
            let existing = fs::read_to_string(&config).unwrap_or_default();
            let filtered: String = existing
                .lines()
                .filter(|l| !l.starts_with("OPENROUTER_API_KEY="))
                .map(|l| format!("{}\n", l))
                .collect();
            fs::write(&config, filtered)?;
            println!("API key cleared.");
        } else {
            println!("No API key saved.");
        }
        return Ok(());
    }

    // check if key already exists and ask to replace
    let existing_key = load_api_key();
    if existing_key.is_some() {
        let replace = Confirm::new("An API key is already saved. Replace it?")
            .with_default(false)
            .prompt()?;
        if !replace {
            println!("Keeping existing API key.");
            return Ok(());
        }
    }

    // loop until a valid key is entered
    let mut input_key = match key {
        Some(k) => k,
        None => Password::new("Enter your OpenRouter API key:")
            .without_confirmation()
            .prompt()?,
    };

    let mut count = 3;

    loop {
        if count <= 0 {
            println!("try another api keys: {}", input_key);
            break Ok(());
        }

        print!("Verifying API key... ");
        if verify_api_key(&input_key).await {
            save_api_key(&config, input_key.trim())?;
            println!("API key saved to {}", config.display());
            return Ok(());
        }
        println!("Invalid API key, try again.");
        input_key = Password::new("Enter your OpenRouter API key:")
            .without_confirmation()
            .prompt()?;

        count -= 1;
    }
}

async fn verify_api_key(key: &str) -> bool {
    let client = reqwest::Client::new();
    let Ok(response) = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "nvidia/nemotron-3-super-120b-a12b:free",
            "messages": [{"role": "user", "content": "hi"}],
            "max_tokens": 1
        }))
        .send()
        .await
    else {
        return false;
    };
    response.status().is_success()
}

fn save_api_key(config: &PathBuf, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(config.parent().unwrap())?;
    let existing = fs::read_to_string(config).unwrap_or_default();
    let mut lines: Vec<String> = existing
        .lines()
        .filter(|l| !l.starts_with("OPENROUTER_API_KEY="))
        .map(String::from)
        .collect();
    lines.push(format!("OPENROUTER_API_KEY={}", key));
    fs::write(config, lines.join("\n") + "\n")?;
    Ok(())
}
