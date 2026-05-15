#[cfg(test)]
mod skill_scheduler_test {
    use crate::skill_loader::SkillTrigger;

    use super::*;
    use langhub::LLMClient;
    use langhub::types::ModelProvider;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_skill() -> Skill {
        Skill {
            name: "test-skill".to_string(),
            description: "A test skill for unit testing".to_string(),
            version: Some("1.0.0".to_string()),
            license: None,
            author: Some("Test Author".to_string()),
            compatibility: None,
            triggers: Some(SkillTrigger {
                patterns: vec!["test".to_string(), "demo".to_string()],
                case_sensitive: false,
            }),
            allowed_tools: vec!["http".to_string()],
            dependencies: vec![],
            metadata: None,
            parameters: vec![],
            instructions: "Do something useful".to_string(),
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
        assert!(list.contains("Version: 1.0.0"));
        assert!(list.contains("Triggers: test, demo"));
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

    #[test]
    fn test_get_skills_prompt() {
        let scheduler = create_test_scheduler();
        let prompt = scheduler.get_skills_prompt();
        assert!(prompt.contains("test-skill"));
        assert!(prompt.contains("A test skill for unit testing"));
        assert!(prompt.contains("Triggers: test, demo"));
    }

    #[test]
    fn test_skill_context_generation() {
        let skill = create_test_skill();
        let scheduler = create_test_scheduler();
        let context = scheduler.get_skill_context(&skill);
        assert!(context.contains("# Skill: test-skill"));
        assert!(context.contains("## Instructions"));
        assert!(context.contains("Do something useful"));
        assert!(context.contains("## Triggers"));
        assert!(context.contains("test, demo"));
    }

    #[test]
    fn test_reload_skills() {
        let temp_dir = tempdir().unwrap();
        let skills_dir = temp_dir.path();

        let skill_subdir = skills_dir.join("reload-skill");
        fs::create_dir(&skill_subdir).unwrap();

        let skill_content = r#"---
name: reload-skill
description: A skill that can be reloaded
version: 1.0.0
---

# Instructions
Do something.
"#;

        fs::write(skill_subdir.join("SKILL.md"), skill_content).unwrap();

        let llm = LLMClient::new(ModelProvider::OpenAI).unwrap();
        let mut scheduler = SkillScheduler::new(vec![], llm);
        assert!(!scheduler.has_skills());

        scheduler
            .reload_skills(skills_dir.to_str().unwrap())
            .unwrap();
        assert!(scheduler.has_skills());
        assert!(scheduler.get_skill("reload-skill").is_some());
    }
}
