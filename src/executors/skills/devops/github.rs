use crate::config::get_config;
use crate::executors::skills::common::Http;
use crate::executors::types::{Skill, SkillParameter};
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;

/// GitHub base helper
struct GitHubApi;

impl GitHubApi {
    fn build_url(endpoint: &str) -> String {
        let config = get_config();
        format!(
            "{}/{}",
            config.github_api_url.trim_end_matches('/'),
            endpoint
        )
    }

    fn build_headers() -> HashMap<String, String> {
        let config = get_config();
        let mut headers = HashMap::new();
        headers.insert(
            "Accept".to_string(),
            "application/vnd.github.v3+json".to_string(),
        );
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", config.github_token),
        );
        headers.insert("User-Agent".to_string(), "Hippox-Engine".to_string());
        headers
    }

    async fn get(endpoint: &str) -> Result<String> {
        let config = get_config();
        let req_config = Http::RequestConfig {
            url: Self::build_url(endpoint),
            method: "GET".to_string(),
            headers: Some(Self::build_headers()),
            body: None,
            timeout_secs: Some(config.github_timeout),
        };
        let response = Http::execute(&req_config).await?;
        if response.is_success {
            Ok(response.body)
        } else {
            anyhow::bail!("GitHub API error: {}", response.body)
        }
    }

    async fn post(endpoint: &str, body: &str) -> Result<String> {
        let config = get_config();
        let req_config = Http::RequestConfig {
            url: Self::build_url(endpoint),
            method: "POST".to_string(),
            headers: Some(Self::build_headers()),
            body: Some(body.to_string()),
            timeout_secs: Some(config.github_timeout),
        };
        let response = Http::execute(&req_config).await?;
        if response.is_success {
            Ok(response.body)
        } else {
            anyhow::bail!("GitHub API error: {}", response.body)
        }
    }

    async fn put(endpoint: &str, body: Option<&str>) -> Result<String> {
        let config = get_config();
        let req_config = Http::RequestConfig {
            url: Self::build_url(endpoint),
            method: "PUT".to_string(),
            headers: Some(Self::build_headers()),
            body: body.map(|s| s.to_string()),
            timeout_secs: Some(config.github_timeout),
        };
        let response = Http::execute(&req_config).await?;
        if response.is_success {
            Ok(response.body)
        } else {
            anyhow::bail!("GitHub API error: {}", response.body)
        }
    }

    async fn delete(endpoint: &str) -> Result<String> {
        let config = get_config();
        let req_config = Http::RequestConfig {
            url: Self::build_url(endpoint),
            method: "DELETE".to_string(),
            headers: Some(Self::build_headers()),
            body: None,
            timeout_secs: Some(config.github_timeout),
        };
        let response = Http::execute(&req_config).await?;
        if response.is_success {
            Ok(response.body)
        } else {
            anyhow::bail!("GitHub API error: {}", response.body)
        }
    }
}

/// Get repository information
#[derive(Debug)]
pub struct GithubGetRepo;

#[async_trait::async_trait]
impl Skill for GithubGetRepo {
    fn name(&self) -> &str {
        "github_get_repo"
    }

    fn description(&self) -> &str {
        "Get information about a GitHub repository"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to get repository details like stars, forks, description"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "owner".to_string(),
                param_type: "string".to_string(),
                description: "Repository owner (username or organization)".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust-lang".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "repo".to_string(),
                param_type: "string".to_string(),
                description: "Repository name".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust".to_string())),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_get_repo",
            "parameters": {
                "owner": "rust-lang",
                "repo": "rust"
            }
        })
    }

    fn example_output(&self) -> String {
        r#"{"name": "rust", "full_name": "rust-lang/rust", "description": "Empowering everyone...", "stargazers_count": 85000, "forks_count": 11000}"#.to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let owner = parameters
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: owner"))?;
        let repo = parameters
            .get("repo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: repo"))?;
        let endpoint = format!("repos/{}/{}", owner, repo);
        GitHubApi::get(&endpoint).await
    }
}

/// Create an issue
#[derive(Debug)]
pub struct GithubCreateIssue;

#[async_trait::async_trait]
impl Skill for GithubCreateIssue {
    fn name(&self) -> &str {
        "github_create_issue"
    }

    fn description(&self) -> &str {
        "Create an issue in a GitHub repository"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to report a bug or request a feature"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "owner".to_string(),
                param_type: "string".to_string(),
                description: "Repository owner".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust-lang".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "repo".to_string(),
                param_type: "string".to_string(),
                description: "Repository name".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "title".to_string(),
                param_type: "string".to_string(),
                description: "Issue title".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("Bug: compilation error".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "body".to_string(),
                param_type: "string".to_string(),
                description: "Issue body/description".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("When compiling with nightly...".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "labels".to_string(),
                param_type: "array".to_string(),
                description: "Labels to apply".to_string(),
                required: false,
                default: Some(Value::Array(vec![])),
                example: Some(json!(["bug", "help-wanted"])),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_create_issue",
            "parameters": {
                "owner": "rust-lang",
                "repo": "rust",
                "title": "Bug: compilation error",
                "body": "When compiling with nightly...",
                "labels": ["bug"]
            }
        })
    }

    fn example_output(&self) -> String {
        r#"{"number": 12345, "html_url": "https://github.com/rust-lang/rust/issues/12345"}"#
            .to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let owner = parameters
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: owner"))?;
        let repo = parameters
            .get("repo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: repo"))?;
        let title = parameters
            .get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: title"))?;
        let mut body = json!({
            "title": title,
        });
        if let Some(b) = parameters.get("body").and_then(|v| v.as_str()) {
            body["body"] = json!(b);
        }
        if let Some(labels) = parameters.get("labels").and_then(|v| v.as_array()) {
            let label_strings: Vec<String> = labels
                .iter()
                .filter_map(|l| l.as_str())
                .map(|s| s.to_string())
                .collect();
            body["labels"] = json!(label_strings);
        }
        let endpoint = format!("repos/{}/{}/issues", owner, repo);
        GitHubApi::post(&endpoint, &body.to_string()).await
    }
}

/// List issues
#[derive(Debug)]
pub struct GithubListIssues;

#[async_trait::async_trait]
impl Skill for GithubListIssues {
    fn name(&self) -> &str {
        "github_list_issues"
    }

    fn description(&self) -> &str {
        "List issues from a GitHub repository"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to see existing issues"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "owner".to_string(),
                param_type: "string".to_string(),
                description: "Repository owner".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust-lang".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "repo".to_string(),
                param_type: "string".to_string(),
                description: "Repository name".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "state".to_string(),
                param_type: "string".to_string(),
                description: "Issue state (open, closed, all)".to_string(),
                required: false,
                default: Some(Value::String("open".to_string())),
                example: Some(Value::String("open".to_string())),
                enum_values: Some(vec![
                    "open".to_string(),
                    "closed".to_string(),
                    "all".to_string(),
                ]),
            },
            SkillParameter {
                name: "limit".to_string(),
                param_type: "integer".to_string(),
                description: "Maximum number of issues to return".to_string(),
                required: false,
                default: Some(Value::Number(30.into())),
                example: Some(Value::Number(10.into())),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_list_issues",
            "parameters": {
                "owner": "rust-lang",
                "repo": "rust",
                "state": "open",
                "limit": 10
            }
        })
    }

    fn example_output(&self) -> String {
        r#"[{"number": 12345, "title": "Bug report", "state": "open"}]"#.to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let owner = parameters
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: owner"))?;
        let repo = parameters
            .get("repo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: repo"))?;
        let state = parameters
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("open");
        let limit = parameters
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(30);
        let endpoint = format!(
            "repos/{}/{}/issues?state={}&per_page={}",
            owner, repo, state, limit
        );
        GitHubApi::get(&endpoint).await
    }
}

/// Star a repository
#[derive(Debug)]
pub struct GithubStarRepo;

#[async_trait::async_trait]
impl Skill for GithubStarRepo {
    fn name(&self) -> &str {
        "github_star_repo"
    }

    fn description(&self) -> &str {
        "Star a GitHub repository"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user wants to star/favorite a repository"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "owner".to_string(),
                param_type: "string".to_string(),
                description: "Repository owner".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust-lang".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "repo".to_string(),
                param_type: "string".to_string(),
                description: "Repository name".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust".to_string())),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_star_repo",
            "parameters": {
                "owner": "rust-lang",
                "repo": "rust"
            }
        })
    }

    fn example_output(&self) -> String {
        "Successfully starred rust-lang/rust".to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let owner = parameters
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: owner"))?;
        let repo = parameters
            .get("repo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: repo"))?;
        let endpoint = format!("user/starred/{}/{}", owner, repo);
        GitHubApi::put(&endpoint, None).await?;
        Ok(format!("Successfully starred {}/{}", owner, repo))
    }
}

/// Search repositories
#[derive(Debug)]
pub struct GithubSearchRepos;

#[async_trait::async_trait]
impl Skill for GithubSearchRepos {
    fn name(&self) -> &str {
        "github_search_repos"
    }

    fn description(&self) -> &str {
        "Search GitHub repositories by query"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to find repositories"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "query".to_string(),
                param_type: "string".to_string(),
                description: "Search query (e.g., 'rust language:rust')".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust language:rust".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "limit".to_string(),
                param_type: "integer".to_string(),
                description: "Maximum number of results".to_string(),
                required: false,
                default: Some(Value::Number(10.into())),
                example: Some(Value::Number(5.into())),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_search_repos",
            "parameters": {
                "query": "rust language:rust",
                "limit": 5
            }
        })
    }

    fn example_output(&self) -> String {
        r#"{"total_count": 12345, "items": [{"full_name": "rust-lang/rust", "description": "..."}]}"#.to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let query = parameters
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;
        let limit = parameters
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10);
        let encoded_query = urlencoding::encode(query);
        let endpoint = format!("search/repositories?q={}&per_page={}", encoded_query, limit);
        GitHubApi::get(&endpoint).await
    }
}

/// Get user information
#[derive(Debug)]
pub struct GithubGetUser;

#[async_trait::async_trait]
impl Skill for GithubGetUser {
    fn name(&self) -> &str {
        "github_get_user"
    }

    fn description(&self) -> &str {
        "Get GitHub user information"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to get profile info of a GitHub user"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "username".to_string(),
            param_type: "string".to_string(),
            description: "GitHub username".to_string(),
            required: true,
            default: None,
            example: Some(Value::String("octocat".to_string())),
            enum_values: None,
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_get_user",
            "parameters": {
                "username": "octocat"
            }
        })
    }

    fn example_output(&self) -> String {
        r#"{"login": "octocat", "name": "The Octocat", "public_repos": 8}"#.to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let username = parameters
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: username"))?;

        let endpoint = format!("users/{}", username);
        GitHubApi::get(&endpoint).await
    }
}

/// Get pull requests
#[derive(Debug)]
pub struct GithubListPRs;

#[async_trait::async_trait]
impl Skill for GithubListPRs {
    fn name(&self) -> &str {
        "github_list_prs"
    }

    fn description(&self) -> &str {
        "List pull requests from a GitHub repository"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user needs to see open pull requests"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "owner".to_string(),
                param_type: "string".to_string(),
                description: "Repository owner".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust-lang".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "repo".to_string(),
                param_type: "string".to_string(),
                description: "Repository name".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("rust".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "state".to_string(),
                param_type: "string".to_string(),
                description: "PR state (open, closed, all)".to_string(),
                required: false,
                default: Some(Value::String("open".to_string())),
                example: Some(Value::String("open".to_string())),
                enum_values: Some(vec![
                    "open".to_string(),
                    "closed".to_string(),
                    "all".to_string(),
                ]),
            },
            SkillParameter {
                name: "limit".to_string(),
                param_type: "integer".to_string(),
                description: "Maximum number of PRs to return".to_string(),
                required: false,
                default: Some(Value::Number(30.into())),
                example: Some(Value::Number(10.into())),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "github_list_prs",
            "parameters": {
                "owner": "rust-lang",
                "repo": "rust",
                "state": "open",
                "limit": 10
            }
        })
    }

    fn example_output(&self) -> String {
        r#"[{"number": 123, "title": "Add feature", "user": {"login": "contributor"}}]"#.to_string()
    }

    fn category(&self) -> &str {
        "github"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let owner = parameters
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: owner"))?;
        let repo = parameters
            .get("repo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: repo"))?;
        let state = parameters
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("open");
        let limit = parameters
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(30);

        let endpoint = format!(
            "repos/{}/{}/pulls?state={}&per_page={}",
            owner, repo, state, limit
        );
        GitHubApi::get(&endpoint).await
    }
}
