use crate::executors::registry;
use crate::t;
use langhub::LLMClient;
use langhub::types::ChatMessage;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SkillScheduler {
    llm: LLMClient,
}

impl SkillScheduler {
    pub fn new(llm: LLMClient) -> Self {
        Self { llm }
    }

    /// Generate a comprehensive prompt with all skill metadata from registry
    pub fn get_skills_prompt(&self) -> String {
        let registry_json = registry::generate_skill_registry_table_json_str();
        format!("## Available Skills (JSON Registry)\n{}", registry_json)
    }

    pub async fn select_skill(&self, user_input: &str) -> anyhow::Result<Option<String>> {
        if registry::list_skills().is_empty() {
            return Ok(None);
        }
        let skills_prompt = self.get_skills_prompt();
        let select_prompt = format!(
            "{}\n\nAvailable skills:\n{}\n\nUser input: {}\n\nRespond with ONLY the skill name, or 'none' if no skill matches.\n",
            t!("prompt.select_skill_header"),
            skills_prompt,
            user_input
        );
        let response = self.llm.generate(&select_prompt).await?;
        let skill_name = response.trim();
        if skill_name == "none" || skill_name.is_empty() {
            Ok(None)
        } else if registry::has_skill(skill_name) {
            Ok(Some(skill_name.to_string()))
        } else {
            Ok(None)
        }
    }

    pub async fn execute(
        &self,
        skill_name: &str,
        user_input: &str,
        conversation_history: &str,
    ) -> anyhow::Result<String> {
        println!("{}", t!("skill.executing", skill_name));
        let skill = registry::get_skill(skill_name)
            .ok_or_else(|| anyhow::anyhow!("Skill not found: {}", skill_name))?;
        let mut parameters = HashMap::new();
        parameters.insert("input".to_string(), Value::String(user_input.to_string()));
        skill.execute(&parameters).await
    }

    pub async fn execute_with_parameters(
        &self,
        skill_name: &str,
        user_input: &str,
        parameters: &HashMap<String, Value>,
        conversation_history: &str,
    ) -> anyhow::Result<String> {
        println!("{}", t!("skill.executing", skill_name));
        let skill = registry::get_skill(skill_name)
            .ok_or_else(|| anyhow::anyhow!("Skill not found: {}", skill_name))?;
        skill.execute(parameters).await
    }

    pub async fn execute_with_messages(
        &self,
        skill_name: &str,
        messages: Vec<ChatMessage>,
    ) -> anyhow::Result<String> {
        let skill = registry::get_skill(skill_name)
            .ok_or_else(|| anyhow::anyhow!("Skill not found: {}", skill_name))?;
        let mut parameters = HashMap::new();
        // Extract content from the last user message
        for msg in messages.iter().rev() {
            if msg.role == "user" {
                parameters.insert("input".to_string(), Value::String(msg.content.clone()));
                break;
            }
        }
        skill.execute(&parameters).await
    }

    pub async fn fallback_chat(&self, user_input: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "{}\n\nYou are a helpful assistant. No specific skill matched the user's request.\n\nUser input: {}\n\nProvide a helpful, natural response to the user.\n",
            t!("prompt.fallback"),
            user_input
        );
        let response = self.llm.generate(&prompt).await?;
        Ok(response)
    }

    pub async fn fallback_chat_with_history(
        &self,
        user_input: &str,
        conversation_history: &str,
    ) -> anyhow::Result<String> {
        let prompt = format!(
            "{}\n\nYou are a helpful assistant. No specific skill matched the user's request.\n\nPrevious conversation:\n{}\n\nUser input: {}\n\nProvide a helpful, natural response considering the conversation history.\n",
            t!("prompt.fallback"),
            conversation_history,
            user_input
        );
        let response = self.llm.generate(&prompt).await?;
        Ok(response)
    }

    pub fn list_skills(&self) -> String {
        let skills = registry::list_skills();
        if skills.is_empty() {
            return t!("skill.no_skills_available").to_string();
        }
        let mut result = String::new();
        for name in skills {
            if let Some(skill) = registry::get_skill(&name) {
                let emoji = match skill.category() {
                    "file" => "📁",
                    "net" => "🌐",
                    "math" => "🔢",
                    "time" => "🕐",
                    "system" => "💻",
                    _ => "⚙️",
                };
                result.push_str(&format!(
                    "   {} - **{}**: {}\n",
                    emoji,
                    name,
                    skill.description()
                ));
            }
        }
        result
    }

    pub fn get_skill_names(&self) -> Vec<String> {
        registry::list_skills()
    }

    pub fn has_skills(&self) -> bool {
        !registry::list_skills().is_empty()
    }

    pub fn get_llm(&self) -> &LLMClient {
        &self.llm
    }
}

#[cfg(test)]
mod skill_scheduler_test {
    use super::*;
    use langhub::LLMClient;
    use langhub::types::ModelProvider;

    fn create_test_scheduler() -> SkillScheduler {
        let llm = LLMClient::new(ModelProvider::OpenAI).unwrap();
        SkillScheduler::new(llm)
    }

    #[test]
    fn test_list_skills() {
        let scheduler = create_test_scheduler();
        let list = scheduler.list_skills();
        // Registry should have at least helloworld skill
        assert!(list.contains("helloworld"));
    }

    #[test]
    fn test_get_skill_names() {
        let scheduler = create_test_scheduler();
        let names = scheduler.get_skill_names();
        assert!(names.contains(&"helloworld".to_string()));
        assert!(names.contains(&"calculator".to_string()));
        assert!(names.contains(&"file_read".to_string()));
    }

    #[test]
    fn test_has_skills() {
        let scheduler = create_test_scheduler();
        assert!(scheduler.has_skills());
    }

    #[test]
    fn test_get_skills_prompt() {
        let scheduler = create_test_scheduler();
        let prompt = scheduler.get_skills_prompt();
        assert!(prompt.contains("Available Skills"));
        assert!(prompt.contains("helloworld"));
        assert!(prompt.contains("calculator"));
    }

    #[tokio::test]
    async fn test_select_skill_with_trigger() {
        let scheduler = create_test_scheduler();
        // This test requires actual LLM call, so we skip it in normal test runs
        // Use integration tests for actual LLM calls
        let result = scheduler.select_skill("calculate 2+3").await;
        assert!(result.is_ok());
    }
}
