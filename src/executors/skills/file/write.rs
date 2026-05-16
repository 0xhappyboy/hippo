use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::executors::{skills::common, types::Skill};

#[derive(Debug)]
pub struct WriteFileSkill;

#[async_trait::async_trait]
impl Skill for WriteFileSkill {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Write content to a file. Parameters: path (required) - file path, content (required) - content to write, append (optional, default false) - append to file"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let path = parameters
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;
        let content = parameters
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'content' parameter"))?;
        let append = parameters
            .get("append")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let validated_path = common::File::validate_path(path, None)?;
        if let Some(parent) = validated_path.parent() {
            common::File::ensure_dir(&parent.to_string_lossy())?;
        }
        if append {
            let existing = if common::File::file_exists(&validated_path.to_string_lossy()) {
                common::File::read_file_content(&validated_path.to_string_lossy())?
            } else {
                String::new()
            };
            let new_content = format!("{}{}", existing, content);
            common::File::write_file_content(
                &validated_path.to_string_lossy(),
                &new_content,
                false,
            )?;
            Ok(format!("Content appended to file: {}", path))
        } else {
            common::File::write_file_content(&validated_path.to_string_lossy(), content, false)?;
            Ok(format!("Content written to file: {}", path))
        }
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: path"))?;
        parameters
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: content"))?;
        Ok(())
    }
}
