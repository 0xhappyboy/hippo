use anyhow::Result;
use hippox::executors::registry::register_skill;
use hippox::executors::types::{Skill, SkillParameter};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

/// A simple skill that echoes back the input message.
///
#[derive(Debug)]
pub struct EchoSkill;

#[async_trait::async_trait]
impl Skill for EchoSkill {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echo back the input message"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when you need to test or echo a message"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "message".to_string(),
                param_type: "string".to_string(),
                description: "The message to echo".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("Hello, World!".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "uppercase".to_string(),
                param_type: "boolean".to_string(),
                description: "Convert message to uppercase".to_string(),
                required: false,
                default: Some(Value::Bool(false)),
                example: Some(Value::Bool(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "echo",
            "parameters": {
                "message": "Hello, World!"
            }
        })
    }

    fn example_output(&self) -> String {
        "Echo: Hello, World!".to_string()
    }

    fn category(&self) -> &str {
        "utility"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let message = parameters
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: message"))?;
        let uppercase = parameters
            .get("uppercase")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let output = if uppercase {
            message.to_uppercase()
        } else {
            message.to_string()
        };
        Ok(format!("Echo: {}", output))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Register the custom skill
    let skill = EchoSkill;
    register_skill("echo".to_string(), Arc::new(skill));
    let skill = hippox::executors::registry::get_skill("echo").unwrap();
    let mut params = HashMap::new();
    params.insert("message".to_string(), json!("Hello, Hippox!"));
    let result = skill.execute(&params).await?;
    println!("Test 1: {}", result);
    let mut params2 = HashMap::new();
    params2.insert("message".to_string(), json!("Hello, Hippox!"));
    params2.insert("uppercase".to_string(), json!(true));
    let result2 = skill.execute(&params2).await?;
    println!("Test 2: {}", result2);
    Ok(())
}
