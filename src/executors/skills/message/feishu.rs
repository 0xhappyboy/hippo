use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    config,
    executors::{RequestConfig, execute, types::{Skill, SkillParameter}},
};

#[derive(Debug)]
pub struct SendFeishuSkill;

#[async_trait::async_trait]
impl Skill for SendFeishuSkill {
    fn name(&self) -> &str {
        "send_feishu"
    }

    fn description(&self) -> &str {
        "Send a message via Feishu (Lark) bot"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user wants to send a Feishu message, notify via Feishu, or send a message to a Feishu group"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
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
                name: "msg_type".to_string(),
                param_type: "string".to_string(),
                description: "Message type: 'text', 'post', or 'image'".to_string(),
                required: false,
                default: Some(Value::String("text".to_string())),
                example: Some(Value::String("post".to_string())),
                enum_values: Some(vec![
                    "text".to_string(),
                    "post".to_string(),
                    "image".to_string(),
                ]),
            },
            SkillParameter {
                name: "title".to_string(),
                param_type: "string".to_string(),
                description: "Title for post messages (required if msg_type is 'post')".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("Notification".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "content".to_string(),
                param_type: "object".to_string(),
                description: "Rich content for post messages (alternative to text)".to_string(),
                required: false,
                default: None,
                example: Some(json!([
                    [{"tag": "text", "text": "Hello "}],
                    [{"tag": "a", "text": "Hippo", "href": "https://hippo.dev"}]
                ])),
                enum_values: None,
            },
            SkillParameter {
                name: "image_key".to_string(),
                param_type: "string".to_string(),
                description: "Image key for image messages (required if msg_type is 'image')"
                    .to_string(),
                required: false,
                default: None,
                example: Some(Value::String("img_v2_xxx".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "at_mobiles".to_string(),
                param_type: "array".to_string(),
                description: "Array of user IDs to @ mention".to_string(),
                required: false,
                default: None,
                example: Some(json!(["ou_xxx", "ou_yyy"])),
                enum_values: None,
            },
            SkillParameter {
                name: "at_all".to_string(),
                param_type: "boolean".to_string(),
                description: "Whether to @ everyone in the group".to_string(),
                required: false,
                default: Some(Value::Bool(false)),
                example: Some(Value::Bool(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "send_feishu",
            "parameters": {
                "text": "Hello from Hippo!"
            }
        })
    }

    fn example_output(&self) -> String {
        "Feishu message sent successfully".to_string()
    }

    fn category(&self) -> &str {
        "messaging"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let text = parameters.get("text").and_then(|v| v.as_str());
        let msg_type = parameters
            .get("msg_type")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
        let title = parameters.get("title").and_then(|v| v.as_str());
        let content = parameters.get("content");
        let image_key = parameters.get("image_key").and_then(|v| v.as_str());
        let at_mobiles = parameters
            .get("at_mobiles")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let at_all = parameters
            .get("at_all")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let cfg = config::get_config();
        let config = config::get_config();
        if !config.is_feishu_configured() {
            anyhow::bail!("Feishu not configured. Please set param: feishu_webhook");
        }
        let webhook = config.feishu_webhook;
        if webhook.is_empty() {
            anyhow::bail!("Feishu not configured. Please set param: feishu_webhook");
        }
        let mut body = serde_json::Map::new();
        match msg_type {
            "post" => {
                let post_title = title.unwrap_or("Notification");
                let post_content = if let Some(c) = content {
                    c.clone()
                } else if let Some(t) = text {
                    json!([[{"tag": "text", "text": t}]])
                } else {
                    json!([[{"tag": "text", "text": ""}]])
                };
                body.insert("msg_type".to_string(), json!("post"));
                body.insert(
                    "content".to_string(),
                    json!({
                        "post": {
                            "zh_cn": {
                                "title": post_title,
                                "content": post_content
                            }
                        }
                    }),
                );
            }
            "image" => {
                let img_key = image_key.ok_or_else(|| {
                    anyhow::anyhow!("Missing 'image_key' parameter for image message")
                })?;
                body.insert("msg_type".to_string(), json!("image"));
                body.insert(
                    "content".to_string(),
                    json!({
                        "image_key": img_key
                    }),
                );
            }
            _ => {
                // Default to text message
                let msg_text = text.unwrap_or("");
                body.insert("msg_type".to_string(), json!("text"));
                body.insert(
                    "content".to_string(),
                    json!({
                        "text": msg_text
                    }),
                );
            }
        }
        if !at_mobiles.is_empty() || at_all {
            let mut at = serde_json::Map::new();
            if !at_mobiles.is_empty() {
                at.insert("atMobiles".to_string(), json!(at_mobiles));
            }
            if at_all {
                at.insert("isAtAll".to_string(), json!(true));
            }
            body.insert("at".to_string(), Value::Object(at));
        }
        let http_config = RequestConfig {
            url: webhook,
            method: "POST".to_string(),
            headers: Some([("Content-Type".to_string(), "application/json".to_string())].into()),
            body: Some(serde_json::to_string(&body)?),
            timeout_secs: Some(30),
        };
        let response = execute(&http_config).await?;
        if response.is_success {
            if let Ok(resp_json) = serde_json::from_str::<Value>(&response.body) {
                if let Some(code) = resp_json.get("code").and_then(|v| v.as_i64()) {
                    if code == 0 {
                        return Ok("Feishu message sent successfully".to_string());
                    } else {
                        let msg = resp_json
                            .get("msg")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown error");
                        return Err(anyhow::anyhow!("Feishu API error: {} - {}", code, msg));
                    }
                }
            }
            Ok("Feishu message sent successfully".to_string())
        } else {
            Err(anyhow::anyhow!(
                "Failed to send Feishu message: {}",
                response.body
            ))
        }
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        let msg_type = parameters
            .get("msg_type")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
        match msg_type {
            "post" => {
                // Post message requires either text or content
                let has_text = parameters.contains_key("text");
                let has_content = parameters.contains_key("content");
                if !has_text && !has_content {
                    anyhow::bail!("Missing required parameter: text or content for post message");
                }
            }
            "image" => {
                parameters
                    .get("image_key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing required parameter: image_key for image message")
                    })?;
            }
            _ => {
                parameters
                    .get("text")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing required parameter: text"))?;
            }
        }
        Ok(())
    }
}
