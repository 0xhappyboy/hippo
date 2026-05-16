use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

/// Skill execution trait
/// All skills must implement this trait
#[async_trait::async_trait]
pub trait Skill: Send + Sync + Debug {
    /// Skill name (must match the `name` field in SKILL.md)
    fn name(&self) -> &str;
    /// Skill description
    fn description(&self) -> &str {
        "No description provided"
    }
    /// Execute the skill with given parameters
    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String>;
    /// Validate parameters before execution
    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        Ok(())
    }
}

/// Skill call instruction parsed from LLM response
///
/// This struct represents the JSON format that LLM should return when it wants to invoke a skill:
/// ```json
/// {
///     "action": "skill_name",
///     "parameters": {
///         "param1": "value1",
///         "param2": "value2"
///     }
/// }
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SkillCall {
    /// Name of the skill to invoke (must match a registered skill name)
    pub action: String,
    /// Parameters to pass to the skill execution function
    #[serde(default)]
    pub parameters: HashMap<String, Value>,
}
