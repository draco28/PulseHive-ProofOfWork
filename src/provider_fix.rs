//! Custom LLM provider that correctly serializes tool_calls in assistant messages
//! to the OpenAI wire format required by GLM and other OpenAI-compatible APIs.
//!
//! The SDK's Message::Assistant serializes tool_calls as {"id", "name", "arguments": Value}
//! but the API expects {"id", "type": "function", "function": {"name", "arguments": "json string"}}.

use async_trait::async_trait;
use futures::Stream;
use pulsehive::prelude::*;
use pulsehive_openai::OpenAIConfig;
use serde::Deserialize;
use serde_json::{json, Value};
use std::pin::Pin;
use std::time::Duration;

pub struct FixedProvider {
    config: OpenAIConfig,
    client: reqwest::Client,
}

impl FixedProvider {
    pub fn new(config: OpenAIConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                if let Ok(val) =
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                {
                    headers.insert(reqwest::header::AUTHORIZATION, val);
                }
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .build()
            .expect("Failed to build HTTP client");

        Self { config, client }
    }

    /// Convert a Message to OpenAI wire format Value, fixing tool_calls.
    fn message_to_wire(msg: &Message) -> Value {
        let mut val = serde_json::to_value(msg).unwrap_or(Value::Null);

        if let Some(obj) = val.as_object_mut() {
            if obj.get("role").and_then(|r| r.as_str()) == Some("assistant") {
                if let Some(tool_calls) = obj.remove("tool_calls") {
                    if let Some(arr) = tool_calls.as_array() {
                        if !arr.is_empty() {
                            let fixed: Vec<Value> = arr
                                .iter()
                                .map(|tc| {
                                    let id = tc["id"].as_str().unwrap_or("").to_string();
                                    let name = tc["name"].as_str().unwrap_or("").to_string();
                                    let arguments =
                                        tc.get("arguments").cloned().unwrap_or(json!({}));
                                    let args_str = if arguments.is_string() {
                                        arguments.as_str().unwrap_or("{}").to_string()
                                    } else {
                                        serde_json::to_string(&arguments)
                                            .unwrap_or_else(|_| "{}".to_string())
                                    };
                                    json!({
                                        "id": id,
                                        "type": "function",
                                        "function": {
                                            "name": name,
                                            "arguments": args_str
                                        }
                                    })
                                })
                                .collect();
                            obj.insert("tool_calls".to_string(), Value::Array(fixed));
                        }
                    }
                }
            }
        }
        val
    }

    fn build_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    #[allow(dead_code)]
    id: String,
    choices: Vec<ChatChoice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: Option<String>,
    tool_calls: Option<Vec<ApiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct ApiToolCall {
    id: String,
    function: ApiFunction,
}

#[derive(Debug, Deserialize)]
struct ApiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming is handled via raw JSON parsing in chat_stream

#[async_trait]
impl LlmProvider for FixedProvider {
    async fn chat(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolDefinition>,
        config: &LlmConfig,
    ) -> Result<LlmResponse> {
        let message_values: Vec<Value> = messages.iter().map(Self::message_to_wire).collect();
        let tool_values = Self::build_tools(&tools);

        let model = if config.model.is_empty() {
            self.config.model.clone()
        } else {
            config.model.clone()
        };

        let mut body = json!({
            "model": model,
            "messages": message_values,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        if !tool_values.is_empty() {
            body["tools"] = Value::Array(tool_values);
        }

        let url = format!("{}/chat/completions", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| PulseHiveError::llm(format!("HTTP request failed: {e}")))?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| PulseHiveError::llm(format!("Failed to read response: {e}")))?;

        if !status.is_success() {
            return Err(PulseHiveError::llm(format!(
                "OpenAI API error (HTTP {}): {}",
                status, text
            )));
        }

        let resp: ChatResponse = serde_json::from_str(&text).map_err(|e| {
            PulseHiveError::llm(format!("Failed to parse response: {e}\nBody: {text}"))
        })?;

        let choice = resp.choices.into_iter().next();
        let (content, tool_calls) = match choice {
            Some(c) => {
                let tcs = c
                    .message
                    .tool_calls
                    .unwrap_or_default()
                    .into_iter()
                    .map(|tc| {
                        let args = serde_json::from_str(&tc.function.arguments)
                            .unwrap_or(Value::Object(serde_json::Map::new()));
                        ToolCall {
                            id: tc.id,
                            name: tc.function.name,
                            arguments: args,
                        }
                    })
                    .collect();
                (c.message.content, tcs)
            }
            None => (None, vec![]),
        };

        let usage = resp.usage.map_or(TokenUsage::default(), |u| TokenUsage {
            input_tokens: u.prompt_tokens,
            output_tokens: u.completion_tokens,
        });

        Ok(LlmResponse {
            content,
            tool_calls,
            usage,
        })
    }

    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolDefinition>,
        config: &LlmConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<LlmChunk>> + Send>>> {
        // Fallback: use non-streaming chat and emit result as a single chunk
        let response = self.chat(messages, tools, config).await?;
        let mut chunks = Vec::new();
        if let Some(content) = response.content {
            chunks.push(Ok(LlmChunk::Text(content)));
        }
        for tc in response.tool_calls {
            chunks.push(Ok(LlmChunk::ToolCallStart {
                id: tc.id.clone(),
                name: tc.name.clone(),
            }));
            let args = serde_json::to_string(&tc.arguments).unwrap_or_default();
            chunks.push(Ok(LlmChunk::ToolCallDelta {
                id: tc.id,
                arguments_delta: args,
            }));
        }
        chunks.push(Ok(LlmChunk::Done));
        Ok(Box::pin(futures::stream::iter(chunks)))
    }
}
