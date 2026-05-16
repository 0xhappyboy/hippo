use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    config,
    executors::{
        skills::common::Http,
        types::{Skill, SkillParameter},
    },
};

#[derive(Debug)]
pub struct SendWecomSkill;

#[async_trait::async_trait]
impl Skill for SendWecomSkill {
    fn name(&self) -> &str {
        "send_wecom"
    }

    fn description(&self) -> &str {
        "Send a message via WeCom (Enterprise WeChat) robot"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user wants to send a WeCom message, notify via WeCom, or send a message to a WeCom group"
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
                description: "Message type: 'text', 'markdown', 'image', 'news'".to_string(),
                required: false,
                default: Some(Value::String("text".to_string())),
                example: Some(Value::String("markdown".to_string())),
                enum_values: Some(vec![
                    "text".to_string(),
                    "markdown".to_string(),
                    "image".to_string(),
                    "news".to_string(),
                ]),
            },
            SkillParameter {
                name: "mentioned_list".to_string(),
                param_type: "array".to_string(),
                description: "Array of user IDs to @ mention, use ['@all'] to mention everyone".to_string(),
                required: false,
                default: None,
                example: Some(json!(["zhangsan", "lisi"])),
                enum_values: None,
            },
            SkillParameter {
                name: "mentioned_mobile_list".to_string(),
                param_type: "array".to_string(),
                description: "Array of mobile numbers to @ mention".to_string(),
                required: false,
                default: None,
                example: Some(json!(["13800000000", "13900000000"])),
                enum_values: None,
            },
            SkillParameter {
                name: "image_base64".to_string(),
                param_type: "string".to_string(),
                description: "Base64 encoded image content (required if msg_type is 'image')".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkSEw8UHRofHh0a...".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "image_md5".to_string(),
                param_type: "string".to_string(),
                description: "MD5 hash of the image (required if msg_type is 'image')".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "news_articles".to_string(),
                param_type: "array".to_string(),
                description: "Array of articles for news message (required if msg_type is 'news')".to_string(),
                required: false,
                default: None,
                example: Some(json!([
                    {
                        "title": "News Title",
                        "description": "News description",
                        "url": "https://example.com",
                        "picurl": "https://example.com/pic.jpg"
                    }
                ])),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "send_wecom",
            "parameters": {
                "text": "Hello from Hippo!"
            }
        })
    }

    fn example_output(&self) -> String {
        "WeCom message sent successfully".to_string()
    }

    fn category(&self) -> &str {
        "messaging"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let text = parameters
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'text' parameter"))?;
        let msg_type = parameters
            .get("msg_type")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
        let mentioned_list = parameters
            .get("mentioned_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mentioned_mobile_list = parameters
            .get("mentioned_mobile_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let cfg = config::get_config();
        if !cfg.is_wecom_configured() {
            anyhow::bail!("WeCom not configured. Please set param: wecom_webhook");
        }
        let webhook = cfg.wecom_webhook;
        let mut body = serde_json::Map::new();
        match msg_type {
            "markdown" => {
                body.insert("msgtype".to_string(), json!("markdown"));
                body.insert(
                    "markdown".to_string(),
                    json!({
                        "content": text
                    }),
                );
            }
            "image" => {
                let image_base64 = parameters
                    .get("image_base64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing 'image_base64' parameter for image message")
                    })?;
                let image_md5 = parameters
                    .get("image_md5")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing 'image_md5' parameter for image message")
                    })?;

                body.insert("msgtype".to_string(), json!("image"));
                body.insert(
                    "image".to_string(),
                    json!({
                        "base64": image_base64,
                        "md5": image_md5
                    }),
                );
            }
            "news" => {
                let articles = parameters
                    .get("news_articles")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing 'news_articles' parameter for news message")
                    })?;

                body.insert("msgtype".to_string(), json!("news"));
                body.insert(
                    "news".to_string(),
                    json!({
                        "articles": articles
                    }),
                );
            }
            _ => {
                // Default to text message
                body.insert("msgtype".to_string(), json!("text"));
                let mut text_content = json!({
                    "content": text
                });

                // Add @ mention support
                if !mentioned_list.is_empty() {
                    text_content["mentioned_list"] = json!(mentioned_list);
                }
                if !mentioned_mobile_list.is_empty() {
                    text_content["mentioned_mobile_list"] = json!(mentioned_mobile_list);
                }

                body.insert("text".to_string(), text_content);
            }
        }
        let http_config = Http::RequestConfig {
            url: webhook,
            method: "POST".to_string(),
            headers: Some([("Content-Type".to_string(), "application/json".to_string())].into()),
            body: Some(serde_json::to_string(&body)?),
            timeout_secs: Some(30),
        };
        let response = Http::execute(&http_config).await?;
        if response.is_success {
            if let Ok(resp_json) = serde_json::from_str::<Value>(&response.body) {
                if let Some(errcode) = resp_json.get("errcode").and_then(|v| v.as_i64()) {
                    if errcode == 0 {
                        return Ok("WeCom message sent successfully".to_string());
                    } else {
                        let errmsg = resp_json
                            .get("errmsg")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown error");
                        return Err(anyhow::anyhow!("WeCom API error: {} - {}", errcode, errmsg));
                    }
                }
            }
            Ok("WeCom message sent successfully".to_string())
        } else {
            Err(anyhow::anyhow!(
                "Failed to send WeCom message: {}",
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
            "image" => {
                parameters
                    .get("image_base64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Missing required parameter: image_base64 for image message"
                        )
                    })?;
                parameters
                    .get("image_md5")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing required parameter: image_md5 for image message")
                    })?;
            }
            "news" => {
                parameters
                    .get("news_articles")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Missing required parameter: news_articles for news message"
                        )
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
