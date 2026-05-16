use crate::executors::{skills::common::Http, types::Skill};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequestSkill;

#[async_trait::async_trait]
impl Skill for HttpRequestSkill {
    fn name(&self) -> &str {
        "http_request"
    }

    fn description(&self) -> &str {
        "Send HTTP requests to web APIs"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = Http::parse_config(parameters)?;
        let response = Http::execute(&config).await?;
        Ok(response.to_formatted_string())
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: url"))?;
        Ok(())
    }
}
