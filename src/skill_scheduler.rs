use crate::skill_loader::Skill;
use crate::t;
use langhub::LLMClient;
use langhub::types::ChatMessage;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SkillScheduler {
    skills: Vec<Skill>,
    llm: LLMClient,
    skill_cache: HashMap<String, Skill>,
}

impl SkillScheduler {
    pub fn new(skills: Vec<Skill>, llm: LLMClient) -> Self {
        let mut skill_cache = HashMap::new();
        for skill in &skills {
            skill_cache.insert(skill.name.clone(), skill.clone());
        }
        Self {
            skills,
            llm,
            skill_cache,
        }
    }

    pub fn get_skills_prompt(&self) -> String {
        self.skills
            .iter()
            .map(|s| format!("- {}: {}", s.name, s.description))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub async fn select_skill(&self, user_input: &str) -> anyhow::Result<Option<&Skill>> {
        if self.skills.is_empty() {
            return Ok(None);
        }
        let skills_prompt = self.get_skills_prompt();
        let select_prompt = format!(
            "{}\n\n{}\n\n{}",
            t!("prompt.select_skill_header"),
            skills_prompt,
            t!("prompt.select_skill_footer", user_input)
        );
        let response = self.llm.generate(&select_prompt).await?;
        let skill_name = response.trim();
        if skill_name == "none" || skill_name.is_empty() {
            Ok(None)
        } else {
            Ok(self.skills.iter().find(|s| s.name == skill_name))
        }
    }

    pub async fn execute(
        &self,
        skill: &Skill,
        user_input: &str,
        conversation_history: &str,
    ) -> anyhow::Result<String> {
        let execution_prompt = format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}",
            t!("prompt.execute_skill_header"),
            t!("prompt.skill_name", &skill.name),
            skill.instructions,
            t!("prompt.previous_conversation", conversation_history),
            t!("prompt.user_input", user_input)
        );
        let response = self.llm.generate(&execution_prompt).await?;
        Ok(response)
    }

    pub async fn execute_with_messages(
        &self,
        skill: &Skill,
        messages: Vec<ChatMessage>,
    ) -> anyhow::Result<String> {
        let system_prompt = format!(
            "{}\n\n{}\n\n{}\n\n{}",
            t!("prompt.system_prompt_header"),
            t!("prompt.skill_name", &skill.name),
            skill.instructions,
            t!("prompt.system_prompt_footer")
        );
        let mut full_messages = vec![ChatMessage::system(&system_prompt)];
        full_messages.extend(messages);
        let response = self.llm.chat(full_messages).await?;
        Ok(response)
    }

    pub async fn fallback_chat(&self, user_input: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "{}\n\n{}",
            t!("prompt.fallback"),
            t!("prompt.user_input", user_input)
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
            "{}\n\n{}\n\n{}",
            t!("prompt.fallback"),
            t!("prompt.previous_conversation", conversation_history),
            t!("prompt.user_input", user_input)
        );
        let response = self.llm.generate(&prompt).await?;
        Ok(response)
    }

    pub fn list_skills(&self) -> String {
        if self.skills.is_empty() {
            t!("skill.no_skills_available").to_string()
        } else {
            self.skills
                .iter()
                .map(|s| format!("   - {}: {}", s.name, s.description))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skill_cache.get(name)
    }

    pub fn get_all_skills(&self) -> &Vec<Skill> {
        &self.skills
    }

    pub fn has_skills(&self) -> bool {
        !self.skills.is_empty()
    }

    pub fn reload_skills(&mut self, skills_dir: &str) -> anyhow::Result<()> {
        let new_skills = crate::skill_loader::SkillLoader::load_all(skills_dir)?;
        self.skills = new_skills;
        self.skill_cache.clear();
        for skill in &self.skills {
            self.skill_cache.insert(skill.name.clone(), skill.clone());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use langhub::LLMClient;
    use langhub::types::ModelProvider;

    fn create_test_skill() -> Skill {
        Skill {
            name: "test-skill".to_string(),
            description: "A test skill for unit testing".to_string(),
            instructions: "Do something useful".to_string(),
            allowed_tools: vec!["http".to_string()],
            path: std::path::PathBuf::new(),
        }
    }

    fn create_test_scheduler() -> SkillScheduler {
        let llm = LLMClient::new(ModelProvider::OpenAI).unwrap();
        let skills = vec![create_test_skill()];
        SkillScheduler::new(skills, llm)
    }

    #[test]
    fn test_list_skills() {
        let scheduler = create_test_scheduler();
        let list = scheduler.list_skills();
        assert!(list.contains("test-skill"));
    }

    #[test]
    fn test_get_skill() {
        let scheduler = create_test_scheduler();
        let skill = scheduler.get_skill("test-skill");
        assert!(skill.is_some());
        assert_eq!(skill.unwrap().name, "test-skill");
    }

    #[test]
    fn test_get_nonexistent_skill() {
        let scheduler = create_test_scheduler();
        let skill = scheduler.get_skill("nonexistent");
        assert!(skill.is_none());
    }

    #[test]
    fn test_has_skills() {
        let scheduler = create_test_scheduler();
        assert!(scheduler.has_skills());

        let llm = LLMClient::new(ModelProvider::OpenAI).unwrap();
        let empty_scheduler = SkillScheduler::new(vec![], llm);
        assert!(!empty_scheduler.has_skills());
    }
}
