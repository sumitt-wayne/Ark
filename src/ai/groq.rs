use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::ai::config;

const GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

pub fn generate(prompt: &str) -> Result<String, String> {
    let ai_config = config::load_config()?;

    // Decrypt API key before use
    let api_key = config::decrypt_key(&ai_config.api_key);

    let client = Client::new();

    let request = GroqRequest {
        model: ai_config.model,
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant for developers. Be concise and practical.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        max_tokens: 500,
        temperature: 0.3,
    };

    let response = client
        .post(GROQ_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .map_err(|e| format!("Failed to connect to Groq: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Groq API error: {}", response.status()));
    }

    let groq_response: GroqResponse = response
        .json()
        .map_err(|e| format!("Failed to parse Groq response: {}", e))?;

    groq_response
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content.trim().to_string())
        .ok_or_else(|| "Empty response from Groq".to_string())
}
