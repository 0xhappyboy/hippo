use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::executors::types::Skill;

#[derive(Debug)]
pub struct HelloWorldSkill;

#[async_trait::async_trait]
impl Skill for HelloWorldSkill {
    fn name(&self) -> &str {
        "helloworld"
    }

    fn description(&self) -> &str {
        "A simple hello world skill"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let name = parameters
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("World");
        println!("{}", format!("Hello, {}!", name));
        Ok(format!("Hello, {}!", name))
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        Ok(())
    }
}
