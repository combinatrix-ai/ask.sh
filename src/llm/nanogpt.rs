use async_trait::async_trait;
use futures::stream::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{ChatStream, LLMConfig, LLMError, LLMProvider};

const NANOGPT_API_URL: &str = "https://nano-gpt.com/api/v1/chat/completions";

#[derive(Debug)]
pub struct NanoGPTProvider {
    client: Client,
    model: String,
    api_key: String,
}

#[derive(Serialize, Debug)]
struct NanoGPTRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    max_tokens: u32,
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct NanoGPTStreamEvent {
    object: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    delta: Option<Delta>,
    // finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Delta {
    content: Option<String>,
}

impl NanoGPTProvider {
    pub fn new(config: LLMConfig) -> Result<Self, LLMError> {
        let client = Client::builder()
            .build()
            .map_err(|e| LLMError::ConfigError(e.to_string()))?;

        Ok(Self {
            client,
            model: config.model,
            api_key: config.api_key,
        })
    }

    fn create_request(&self, system_message: &str, user_message: &str) -> NanoGPTRequest {
        NanoGPTRequest {
            model: self.model.clone(),
            messages: vec![
            Message {
                role: "system".to_string(),
                content: system_message.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            }],
            stream: true,
            max_tokens: 4096,
        }
    }

    fn parse_sse_line(line: &str) -> Option<String> {
        if line.is_empty() || line.starts_with(':') {
            return None;
        }
        
        if let Some(data) = line.strip_prefix("data: ") {
            let event = serde_json::from_str::<NanoGPTStreamEvent>(data).ok()?;
            if event.object != "chat.completion.chunk" {
                return None;
            }
        
            let choice = event.choices.get(0)?;
            if let Some(delta) = &choice.delta {
                if let Some(content) = &delta.content {
                    return Some(content.clone());
                }
            }
        }
        None
    }
}

#[async_trait]
impl LLMProvider for NanoGPTProvider {
    fn name(&self) -> &'static str {
        "nanogpt"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn chat_stream(
        &self,
        system_message: String,
        user_message: String,
    ) -> Result<ChatStream, LLMError> {
        let request = self.create_request(&system_message, &user_message);

        let response = self
            .client
            .post(NANOGPT_API_URL)
            .header(header::CONTENT_TYPE, "application/json")
            .header("authorization", format!("Bearer {}", &self.api_key))
            .header("accept", "text/event-stream")
            .json(&request)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LLMError::ApiError(format!(
                "NanoGPT API error: {}",
                error_text
            )));
        }

        let stream = response.bytes_stream().map(move |result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut content = String::new();

                for line in text.lines() {
                    if let Some(text) = Self::parse_sse_line(line) {
                        content.push_str(&text);
                    }
                }

                if !content.is_empty() {
                    Ok(content)
                } else {
                    Ok(String::new())
                }
            }
            Err(e) => Err(LLMError::NetworkError(e.to_string())),
        });

        let filtered_stream = stream.filter(|result| {
            futures::future::ready(match result {
                Ok(content) => !content.is_empty(),
                Err(_) => true,
            })
        });

        Ok(Box::pin(filtered_stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nanogpt_provider_creation() {
        let config = LLMConfig {
            provider: "nanogpt".to_string(),
            model: "gpt-4o".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
        };

        let provider = NanoGPTProvider::new(config).unwrap();
        assert_eq!(provider.name(), "nanogpt");
        assert_eq!(provider.model(), "gpt-4o");
    }
}
