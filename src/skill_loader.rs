use crate::t;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub allowed_tools: Vec<String>,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    name: String,
    description: String,
    #[serde(default)]
    allowed_tools: Vec<String>,
}

pub struct SkillLoader;

impl SkillLoader {
    pub fn load_all(skills_dir: &str) -> anyhow::Result<Vec<Skill>> {
        let mut skills = Vec::new();
        let skills_path = Path::new(skills_dir);
        if !skills_path.exists() {
            anyhow::bail!(t!("error.config_not_found", skills_dir));
        }
        for entry in walkdir::WalkDir::new(skills_path)
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir() {
                let skill_file = path.join("SKILL.md");
                if skill_file.exists() {
                    if let Ok(skill) = Self::parse_skill_file(&skill_file) {
                        skills.push(skill);
                    }
                }
            }
        }
        Ok(skills)
    }

    pub fn load_by_name(skills_dir: &str, name: &str) -> anyhow::Result<Option<Skill>> {
        let skill_path = Path::new(skills_dir).join(name).join("SKILL.md");
        if skill_path.exists() {
            Ok(Some(Self::parse_skill_file(&skill_path)?))
        } else {
            Ok(None)
        }
    }

    fn parse_skill_file(path: &Path) -> anyhow::Result<Skill> {
        let content = fs::read_to_string(path)?;
        let (frontmatter, instructions) = Self::parse_frontmatter(&content)?;
        Ok(Skill {
            name: frontmatter.name,
            description: frontmatter.description,
            instructions,
            allowed_tools: frontmatter.allowed_tools,
            path: path.to_path_buf(),
        })
    }

    fn parse_frontmatter(content: &str) -> anyhow::Result<(SkillFrontmatter, String)> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            anyhow::bail!(t!("error.invalid_skill_format"));
        }
        let frontmatter: SkillFrontmatter = serde_yaml::from_str(parts[1])?;
        let instructions = parts[2].trim().to_string();
        Ok((frontmatter, instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_valid_skill() {
        let content = r#"---
name: test-skill
description: A test skill
allowed_tools: [http, fs]
---

# Test Instructions

Follow these steps:
1. Do something
2. Return result
"#;

        let (frontmatter, instructions) = SkillLoader::parse_frontmatter(content).unwrap();

        assert_eq!(frontmatter.name, "test-skill");
        assert_eq!(frontmatter.description, "A test skill");
        assert_eq!(frontmatter.allowed_tools, vec!["http", "fs"]);
        assert!(instructions.contains("Test Instructions"));
    }

    #[test]
    fn test_parse_skill_without_allowed_tools() {
        let content = r#"---
name: simple-skill
description: A simple skill
---

# Simple Instructions
"#;

        let (frontmatter, _) = SkillLoader::parse_frontmatter(content).unwrap();

        assert_eq!(frontmatter.name, "simple-skill");
        assert!(frontmatter.allowed_tools.is_empty());
    }

    #[test]
    fn test_parse_invalid_skill() {
        let content = "No frontmatter here";
        let result = SkillLoader::parse_frontmatter(content);
        assert!(result.is_err());
    }
}
