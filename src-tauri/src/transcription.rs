use reqwest::multipart;
use serde::{Deserialize, Serialize};

pub fn whisper_prompt() -> String {
    const TERMS: &[&str] = &[
        "Cursor",
        "Composer",
        "agent",
        "prompt",
        "vibe coding",
        "refactor",
        "implement",
        "fix",
        "debug",
        "bug",
        "explain",
        "optimize",
        "clean up",
        "add tests",
        "unit tests",
        "integration tests",
        "component",
        "hook",
        "state",
        "props",
        "API",
        "database",
        "schema",
        "migration",
        "TypeScript",
        "JavaScript",
        "React",
        "Next.js",
        "Tailwind",
        "Tauri",
        "Rust",
        "Node.js",
        "frontend",
        "backend",
        "terminal",
        "PowerShell",
        "GitHub",
        "README",
    ];

    TERMS.join(", ")
}

pub fn is_hallucination(text: &str) -> bool {
    let normalized = normalize_transcript(text);
    if normalized.is_empty() {
        return true;
    }

    const HALLUCINATIONS: &[&str] = &[
        "vielen dank",
        "danke",
        "danke schon",
        "danke schoen",
        "danke fur das zuschauen",
        "danke furs zuschauen",
        "danke fur das zuhorren",
        "danke furs zuhorren",
        "bis zum nachsten mal",
        "thank you",
        "thanks",
        "thanks for watching",
        "thanks for listening",
        "thank you for watching",
        "thank you for listening",
        "like and subscribe",
        "subscribe",
        "untertitel",
        "subtitles by",
        "you",
        "bye",
        "goodbye",
        "okay",
        "ok",
        "hmm",
        "mh",
        "ah",
        "oh",
        "the end",
        "silence",
        "music",
        "applause",
        "дякую",
        "дякуємо",
        "дякую за перегляд",
        "субтитри",
        "до побачення",
    ];

    HALLUCINATIONS.contains(&normalized.as_str())
}

fn normalize_transcript(text: &str) -> String {
    text.trim()
        .trim_matches(|character: char| {
            character.is_ascii_punctuation() || character.is_whitespace()
        })
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn should_insert_transcript(text: &str, duration_ms: u32, rms: f32) -> bool {
    let normalized = normalize_transcript(text);
    if normalized.is_empty() {
        return false;
    }

    if is_hallucination(text) {
        return false;
    }

    let has_speech =
        duration_ms >= crate::audio::MIN_SPEECH_DURATION_MS && rms >= crate::audio::MIN_SPEECH_RMS;

    // Whisper often hallucinates short polite phrases on near-silent audio.
    if normalized.len() <= 24 && !has_speech {
        return false;
    }

    true
}

pub fn is_translate_language(language: &str) -> bool {
    matches!(
        language.trim().to_lowercase().as_str(),
        "de-en" | "de_en" | "de->en" | "translate"
    )
}

pub async fn translate(audio_data: Vec<u8>, api_key: &str) -> Result<String, String> {
    let german_text = transcribe(audio_data, "de", api_key).await?;
    translate_text_to_english(&german_text, api_key).await
}

async fn translate_text_to_english(text: &str, api_key: &str) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(String::new());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let request = ChatCompletionRequest {
        model: "llama-3.1-8b-instant",
        temperature: 0.0,
        max_tokens: 1024,
        messages: vec![
            ChatMessage {
                role: "system",
                content: "Translate German developer instructions into natural, concise English. Return only the translated text.",
            },
            ChatMessage {
                role: "user",
                content: text,
            },
        ],
    };

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Groq API error {status}: {body}"));
    }

    let body = response
        .json::<ChatCompletionResponse>()
        .await
        .map_err(|e| e.to_string())?;

    body.choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .filter(|text| !text.is_empty())
        .ok_or_else(|| "Groq translation returned no text".to_string())
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

pub async fn transcribe(
    audio_data: Vec<u8>,
    language: &str,
    api_key: &str,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let part = multipart::Part::bytes(audio_data)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| e.to_string())?;

    let mut form = multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-large-v3-turbo")
        .text("response_format", "text")
        .text("temperature", "0");

    if !is_auto_language(language) {
        form = form.text("language", language.to_string());
    }

    let prompt = whisper_prompt();
    if !prompt.is_empty() {
        form = form.text("prompt", prompt);
    }

    let response = client
        .post("https://api.groq.com/openai/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Groq API error {status}: {body}"));
    }

    response.text().await.map_err(|e| e.to_string())
}

fn is_auto_language(language: &str) -> bool {
    matches!(
        language.trim().to_lowercase().as_str(),
        "" | "auto" | "automatic"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_common_silent_hallucinations() {
        assert!(is_hallucination("Vielen Dank."));
        assert!(is_hallucination("Thank you"));
        assert!(is_hallucination("  ...  "));
    }

    #[test]
    fn keeps_real_speech() {
        assert!(!is_hallucination("const userName = useState()"));
        assert!(!is_hallucination("vielen dank fur die erklarung"));
    }

    #[test]
    fn rejects_hallucination_on_silent_recording() {
        assert!(!should_insert_transcript("const foo = bar", 120, 0.001));
        assert!(!should_insert_transcript("Vielen Dank.", 120, 0.001));
    }

    #[test]
    fn detects_translate_language_modes() {
        assert!(is_translate_language("de-en"));
        assert!(is_translate_language("DE-EN"));
        assert!(is_translate_language("de_en"));
        assert!(!is_translate_language("de"));
        assert!(!is_translate_language("auto"));
    }
}

pub async fn test_api_key(api_key: &str) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get("https://api.groq.com/openai/v1/models")
        .bearer_auth(api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Invalid API key ({})", response.status()))
    }
}
