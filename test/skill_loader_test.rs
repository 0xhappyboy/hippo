#[cfg(test)]
mod skill_loader_test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_complete_skill() {
        let content = r#"---
name: web-search
description: Search the web using a search engine
version: 1.0.0
author: Lobster Team
license: MIT
compatibility: ">=1.0.0"
triggers:
  patterns:
    - "search for"
    - "find online"
    - "google"
  case_sensitive: false
allowed_tools:
  - http
  - json
dependencies:
  - url-parser
metadata:
  author: lobster
  version: 1.0.0
  emoji: 🔍
  os:
    - linux
    - macos
  requires:
    api_key: true
parameters:
  - name: query
    type: string
    description: The search query
    required: true
  - name: limit
    type: integer
    description: Maximum number of results
    required: false
    default: 10
---

# Web Search Skill

This skill performs web searches.

## Steps

1. Parse the user's search query
2. Call the search API with the query
3. Return formatted results

## Examples

User: "search for Rust programming"
Response: Here are the top 10 results...

## Error Handling

If the API fails, return a friendly error message.
"#;
        let (frontmatter, instructions) = SkillLoader::parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.name, "web-search");
        assert_eq!(
            frontmatter.description,
            "Search the web using a search engine"
        );
        assert_eq!(frontmatter.version, Some("1.0.0".to_string()));
        assert_eq!(frontmatter.author, Some("Lobster Team".to_string()));
        assert!(instructions.contains("Web Search Skill"));
        assert!(instructions.contains("## Steps"));
        assert!(instructions.contains("## Examples"));
        let triggers = frontmatter.triggers.unwrap();
        assert_eq!(triggers.patterns.len(), 3);
        assert!(!triggers.case_sensitive);
        assert_eq!(frontmatter.allowed_tools, vec!["http", "json"]);
        assert_eq!(frontmatter.dependencies, vec!["url-parser"]);
        assert_eq!(frontmatter.parameters.len(), 2);
        assert_eq!(frontmatter.parameters[0].name, "query");
        assert_eq!(frontmatter.parameters[0].param_type, "string");
        assert!(frontmatter.parameters[0].required);
    }

    #[test]
    fn test_parse_minimal_skill() {
        let content = r#"---
name: simple-skill
description: A simple test skill
---

# Simple Instructions

Just do something simple.
"#;

        let (frontmatter, instructions) = SkillLoader::parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.name, "simple-skill");
        assert_eq!(frontmatter.description, "A simple test skill");
        assert!(instructions.contains("Simple Instructions"));
        assert!(frontmatter.version.is_none());
        assert!(frontmatter.allowed_tools.is_empty());
        assert!(frontmatter.parameters.is_empty());
    }

    #[test]
    fn test_parse_skill_with_triggers() {
        let content = r#"---
name: calculator
description: Perform mathematical calculations
triggers:
  patterns:
    - "calculate"
    - "what is"
    - "compute"
    - "math"
  case_sensitive: false
allowed_tools:
  - math
---

# Calculator Skill

Perform calculations based on user input.
"#;

        let (frontmatter, _) = SkillLoader::parse_frontmatter(content).unwrap();

        let triggers = frontmatter.triggers.unwrap();
        assert_eq!(
            triggers.patterns,
            vec!["calculate", "what is", "compute", "math"]
        );
        assert!(!triggers.case_sensitive);
    }

    #[test]
    fn test_parse_skill_with_parameters() {
        let content = r#"---
name: file-processor
description: Process files in various formats
parameters:
  - name: file_path
    type: string
    description: Path to the file
    required: true
  - name: format
    type: string
    description: Output format
    required: false
    default: json
  - name: verbose
    type: boolean
    description: Enable verbose output
    required: false
    default: false
---

# File Processor

Process the file according to parameters.
"#;

        let (frontmatter, _) = SkillLoader::parse_frontmatter(content).unwrap();

        assert_eq!(frontmatter.parameters.len(), 3);
        assert_eq!(frontmatter.parameters[0].name, "file_path");
        assert_eq!(frontmatter.parameters[0].param_type, "string");
        assert!(frontmatter.parameters[0].required);
        assert_eq!(
            frontmatter.parameters[1].default,
            Some(serde_json::Value::String("json".to_string()))
        );
        assert_eq!(frontmatter.parameters[2].param_type, "boolean");
    }

    #[test]
    fn test_parse_skill_with_metadata() {
        let content = r#"---
name: data-analyzer
description: Analyze data and generate reports
metadata:
  author: data-team
  version: 2.1.0
  emoji: 📊
  os:
    - windows
    - linux
  requires:
    python: ">=3.8"
    memory: "4GB"
---

# Data Analyzer

Analyze the provided data.
"#;

        let (frontmatter, _) = SkillLoader::parse_frontmatter(content).unwrap();

        let metadata = frontmatter.metadata.unwrap();
        assert_eq!(metadata.author, Some("data-team".to_string()));
        assert_eq!(metadata.version, Some("2.1.0".to_string()));
        assert_eq!(metadata.emoji, Some("📊".to_string()));
        assert_eq!(
            metadata.os,
            Some(vec!["windows".to_string(), "linux".to_string()])
        );

        let requires = metadata.requires.unwrap();
        assert_eq!(
            requires.get("python"),
            Some(&serde_json::Value::String(">=3.8".to_string()))
        );
        assert_eq!(
            requires.get("memory"),
            Some(&serde_json::Value::String("4GB".to_string()))
        );
    }

    #[test]
    fn test_parse_invalid_skill() {
        let content = "No frontmatter here at all";
        let result = SkillLoader::parse_frontmatter(content);
        assert!(result.is_err());
    }
}
