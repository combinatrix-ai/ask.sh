use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_trait::async_trait;
use futures::stream::StreamExt;
use std::fmt::Debug;

use super::{ChatStream, LLMConfig, LLMError, LLMProvider};

#[derive(Debug)]
pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAIProvider {
    pub fn new(config: LLMConfig) -> Result<Self, LLMError> {
        let mut openai_config = OpenAIConfig::new().with_api_key(config.api_key);

        // Set custom base_url if specified
        if let Some(base_url) = config.base_url {
            openai_config = openai_config.with_api_base(&base_url);
        }

        let client = Client::with_config(openai_config);

        Ok(Self {
            client,
            model: config.model,
        })
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn chat_stream(
        &self,
        system_message: String,
        user_message: String,
    ) -> Result<ChatStream, LLMError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_message.as_str())
                    .build()
                    .map_err(|e| LLMError::InvalidRequestError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message.as_str())
                    .build()
                    .map_err(|e| LLMError::InvalidRequestError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| LLMError::InvalidRequestError(e.to_string()))?;

        let stream = self
            .client
            .chat()
            .create_stream(request)
            .await
            .map_err(|e| LLMError::ApiError(e.to_string()))?;

        // Convert OpenAI stream to a stream using LLMError
        let mapped_stream = stream.map(|result| match result {
            Ok(response) => {
                let content = response
                    .choices
                    .iter()
                    .filter_map(|choice| choice.delta.content.as_ref())
                    .fold(String::new(), |mut acc, s| {
                        acc.push_str(s);
                        acc
                    });
                Ok(content)
            }
            Err(err) => Err(LLMError::ApiError(err.to_string())),
        });

        Ok(Box::pin(mapped_stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai_provider_creation() {
        let config = LLMConfig {
            provider: "openai".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
        };

        let provider = OpenAIProvider::new(config).unwrap();
        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.model(), "gpt-3.5-turbo");
    }
}
