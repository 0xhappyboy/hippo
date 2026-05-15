use crate::t;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Skill parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

/// Skill trigger patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrigger {
    pub patterns: Vec<String>,
    #[serde(default)]
    pub case_sensitive: bool,
}

/// Metadata fields supporting arbitrary JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub author: Option<String>,
    pub version: Option<String>,
    pub emoji: Option<String>,
    pub os: Option<Vec<String>>,
    pub requires: Option<HashMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Complete Skill structure matching Lobster spec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    // Required fields
    pub name: String,
    pub description: String,
    // Optional fields
    pub version: Option<String>,
    pub license: Option<String>,
    pub author: Option<String>,
    pub compatibility: Option<String>,
    // Trigger conditions
    pub triggers: Option<SkillTrigger>,
    // Tool permissions
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    // Dependencies on other skills
    #[serde(default)]
    pub dependencies: Vec<String>,
    // Metadata
    pub metadata: Option<SkillMetadata>,
    // Parameter definitions
    #[serde(default)]
    pub parameters: Vec<SkillParameter>,
    // Main instructions content (supports full Markdown structure)
    pub instructions: String,
    // File path
    pub path: PathBuf,
}

/// Frontmatter intermediate parsing structure
#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    // Required
    name: String,
    description: String,
    // Optional base fields
    version: Option<String>,
    license: Option<String>,
    author: Option<String>,
    compatibility: Option<String>,
    // Triggers
    triggers: Option<SkillTrigger>,
    // Tool permissions
    #[serde(default)]
    allowed_tools: Vec<String>,
    // Dependencies
    #[serde(default)]
    dependencies: Vec<String>,
    // Metadata
    metadata: Option<SkillMetadata>,
    // Parameter definitions
    #[serde(default)]
    parameters: Vec<SkillParameter>,
    // Allow extra unknown fields for forward compatibility
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

pub struct SkillLoader;

impl SkillLoader {
    /// Load all skills from directory
    pub fn load_all(skills_dir: &str) -> anyhow::Result<Vec<Skill>> {
        let mut skills = Vec::new();
        let skills_path = Path::new(skills_dir);
        if !skills_path.exists() {
            anyhow::bail!(t!("error.config_not_found", skills_dir));
        }
        for entry in WalkDir::new(skills_path)
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

    /// Load a single skill by name
    pub fn load_by_name(skills_dir: &str, name: &str) -> anyhow::Result<Option<Skill>> {
        let skill_path = Path::new(skills_dir).join(name).join("SKILL.md");
        if skill_path.exists() {
            Ok(Some(Self::parse_skill_file(&skill_path)?))
        } else {
            Ok(None)
        }
    }

    /// Parse a SKILL.md file
    fn parse_skill_file(path: &Path) -> anyhow::Result<Skill> {
        let content = fs::read_to_string(path)?;
        let (frontmatter, instructions) = Self::parse_frontmatter(&content)?;
        Ok(Skill {
            name: frontmatter.name,
            description: frontmatter.description,
            version: frontmatter.version,
            license: frontmatter.license,
            author: frontmatter.author,
            compatibility: frontmatter.compatibility,
            triggers: frontmatter.triggers,
            allowed_tools: frontmatter.allowed_tools,
            dependencies: frontmatter.dependencies,
            metadata: frontmatter.metadata,
            parameters: frontmatter.parameters,
            instructions,
            path: path.to_path_buf(),
        })
    }

    /// Parse frontmatter from markdown content
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
