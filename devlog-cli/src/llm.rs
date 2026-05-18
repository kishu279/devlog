use crate::cmd::api::load_api_key;
use serde_json::json;

const OPENROUTER_CHAT_COMPLETIONS_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";

pub async fn call_claude(context: &str) -> String {
    let api_key = match load_api_key() {
        Some(k) => k,
        None => return "API key not set. Run `devlog api` to configure it.".to_string(),
    };

    match call_llm(context.to_string(), &api_key).await {
        Ok(response) => response,
        Err(error) => format!("failed to call Claude: {error}"),
    }
}

pub fn request_summary(context: &str) -> String {
    format!(
        "POST {OPENROUTER_CHAT_COMPLETIONS_URL}\nmodel: {MODEL}\nmessages: system prompt + user context ({} chars)\nauthorization: Bearer <redacted>",
        context.chars().count()
    )
}

pub async fn call_llm(
    context: String,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let system_prompt = r#"You are a developer's personal scribe.
Below is a structured log of a developer's actual activity today.
Summarize it into the following three outputs:
1. STANDUP (3 bullets max - Yesterday / Today / Blockers)
   - Be specific. Reference file names, commit messages, test results as evidence.
   - Write in first person, past tense for yesterday/today.
2. DEVLOG (2-4 sentences, narrative form)
   - This is a personal journal entry. Capture what was hard, what was learned.
   - Tone: honest and technical, like a message to a future self.
3. KEY INSIGHT (one sentence)
   - What was the hardest single problem worked on today?
Do not invent information. Only use what is in the activity log below."#;
    let response = client
        .post(OPENROUTER_CHAT_COMPLETIONS_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": MODEL,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": context}
            ],
            "max_tokens": 1000
        }))
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    let response_json = serde_json::from_str::<serde_json::Value>(&body).map_err(|error| {
        boxed_error(format!("invalid JSON from OpenRouter ({status}): {error}"))
    })?;

    if !status.is_success() {
        let message = response_json["error"]["message"]
            .as_str()
            .unwrap_or("unknown OpenRouter error");
        return Err(boxed_error(format!(
            "OpenRouter returned {status}: {message}"
        )));
    }

    if let Some(message) = response_json["error"]["message"].as_str() {
        return Err(boxed_error(format!("OpenRouter error: {message}")));
    }

    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            boxed_error(format!(
                "OpenRouter response missing choices[0].message.content: {response_json}"
            ))
        })?;
    Ok(content.to_string())
}

fn boxed_error(message: String) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, message))
}
