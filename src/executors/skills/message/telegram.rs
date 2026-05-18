use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    config,
    executors::{
        RequestConfig, execute,
        types::{Skill, SkillParameter},
    },
};

#[derive(Debug)]
pub struct SendTelegramSkill;

#[async_trait::async_trait]
impl Skill for SendTelegramSkill {
    fn name(&self) -> &str {
        "send_telegram"
    }

    fn description(&self) -> &str {
        "Send a message via Telegram Bot"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user wants to send a Telegram message, notify via Telegram, or send a message to a Telegram chat"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "chat_id".to_string(),
                param_type: "string".to_string(),
                description: "Telegram chat ID (user or group)".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("123456789".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "text".to_string(),
                param_type: "string".to_string(),
                description: "Message text to send".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("Hello from Hippo!".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "parse_mode".to_string(),
                param_type: "string".to_string(),
                description: "Message parse mode: 'HTML', 'MarkdownV2', or 'Markdown'".to_string(),
                required: false,
                default: Some(Value::String("HTML".to_string())),
                example: Some(Value::String("Markdown".to_string())),
                enum_values: Some(vec![
                    "HTML".to_string(),
                    "MarkdownV2".to_string(),
                    "Markdown".to_string(),
                ]),
            },
            SkillParameter {
                name: "disable_notification".to_string(),
                param_type: "boolean".to_string(),
                description: "Send silently without notification sound".to_string(),
                required: false,
                default: Some(Value::Bool(false)),
                example: Some(Value::Bool(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "send_telegram",
            "parameters": {
                "chat_id": "123456789",
                "text": "Hello from Hippo!"
            }
        })
    }

    fn example_output(&self) -> String {
        "Telegram message sent successfully to chat 123456789".to_string()
    }

    fn category(&self) -> &str {
        "messaging"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let chat_id = parameters
            .get("chat_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'chat_id' parameter"))?;
        let text = parameters
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'text' parameter"))?;
        let parse_mode = parameters
            .get("parse_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("HTML");
        let disable_notification = parameters
            .get("disable_notification")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let config = config::get_config();
        if !config.is_telegram_configured() {
            anyhow::bail!("Telegram not configured. Please set param: telegram_bot_token");
        }
        let bot_token = config.telegram_bot_token;
        let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
        let mut body = HashMap::new();
        body.insert("chat_id".to_string(), json!(chat_id));
        body.insert("text".to_string(), json!(text));
        body.insert("parse_mode".to_string(), json!(parse_mode));
        body.insert(
            "disable_notification".to_string(),
            json!(disable_notification),
        );
        let http_config = RequestConfig {
            url,
            method: "POST".to_string(),
            headers: Some([("Content-Type".to_string(), "application/json".to_string())].into()),
            body: Some(serde_json::to_string(&body)?),
            timeout_secs: Some(30),
        };
        let response = execute(&http_config).await?;
        if response.is_success {
            Ok(format!(
                "Telegram message sent successfully to chat {}",
                chat_id
            ))
        } else {
            Err(anyhow::anyhow!(
                "Failed to send Telegram message: {}",
                response.body
            ))
        }
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("chat_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: chat_id"))?;
        parameters
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: text"))?;
        Ok(())
    }
}
