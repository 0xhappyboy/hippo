//! Docker container management utilities.
//!
//! This module provides skills for Docker operations:
//! - `DockerPsSkill`: List Docker containers
//! - `DockerStartStopSkill`: Start or stop containers
//! - `DockerLogsSkill`: View container logs
//! - `DockerInspectSkill`: Get container details
//! - `DockerExecSkill`: Execute commands in containers

use crate::config::get_config;
use crate::executors::types::{Skill, SkillParameter};
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process::Command;

/// A skill for listing Docker containers.
#[derive(Debug)]
pub struct DockerPsSkill;

#[async_trait::async_trait]
impl Skill for DockerPsSkill {
    fn name(&self) -> &str {
        "docker_ps"
    }

    fn description(&self) -> &str {
        "List Docker containers"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when you need to see running containers, check container status, or find container IDs"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "all".to_string(),
                param_type: "boolean".to_string(),
                description: "Show all containers (including stopped)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "filter".to_string(),
                param_type: "string".to_string(),
                description: "Filter output (e.g., 'status=exited', 'name=myapp')".to_string(),
                required: false,
                default: None,
                example: Some(json!("status=running")),
                enum_values: None,
            },
            SkillParameter {
                name: "format".to_string(),
                param_type: "string".to_string(),
                description: "Output format: table, json, or quiet".to_string(),
                required: false,
                default: Some(json!("table")),
                example: Some(json!("json")),
                enum_values: Some(vec![
                    "table".to_string(),
                    "json".to_string(),
                    "quiet".to_string(),
                ]),
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "docker_ps",
            "parameters": {
                "all": true
            }
        })
    }

    fn example_output(&self) -> String {
        "CONTAINER ID   IMAGE     COMMAND   STATUS          PORTS     NAMES\nabc123def456   nginx     \"nginx\"   Up 2 hours      80/tcp    web_nginx".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let all = parameters
            .get("all")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let filter = parameters.get("filter").and_then(|v| v.as_str());
        let format = parameters
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("table");
        let mut cmd = Command::new("docker");
        if config.is_docker_configured() {
            cmd.env("DOCKER_HOST", config.get_docker_host());
        }
        cmd.arg("ps");
        if all {
            cmd.arg("-a");
        }
        if let Some(f) = filter {
            cmd.arg("--filter").arg(f);
        }
        match format {
            "json" => {
                cmd.arg("--format").arg("json");
            }
            "quiet" => {
                cmd.arg("-q");
            }
            _ => {}
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Docker ps failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if format == "json" {
            let containers: Vec<serde_json::Value> = stdout
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
            Ok(serde_json::to_string_pretty(&containers)?)
        } else {
            Ok(stdout.to_string())
        }
    }
}

/// A skill for starting or stopping Docker containers.
#[derive(Debug)]
pub struct DockerStartStopSkill;

#[async_trait::async_trait]
impl Skill for DockerStartStopSkill {
    fn name(&self) -> &str {
        "docker_start_stop"
    }

    fn description(&self) -> &str {
        "Start, stop, restart, or pause Docker containers"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to control container lifecycle: start, stop, restart, pause, or unpause"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name or ID".to_string(),
                required: true,
                default: None,
                example: Some(json!("my_container")),
                enum_values: None,
            },
            SkillParameter {
                name: "action".to_string(),
                param_type: "string".to_string(),
                description: "Action to perform: start, stop, restart, pause, unpause".to_string(),
                required: true,
                default: None,
                example: Some(json!("restart")),
                enum_values: Some(vec![
                    "start".to_string(),
                    "stop".to_string(),
                    "restart".to_string(),
                    "pause".to_string(),
                    "unpause".to_string(),
                ]),
            },
            SkillParameter {
                name: "timeout".to_string(),
                param_type: "integer".to_string(),
                description: "Timeout in seconds for stop (default: 10)".to_string(),
                required: false,
                default: Some(json!(10)),
                example: Some(json!(30)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "docker_start_stop",
            "parameters": {
                "container": "redis",
                "action": "restart"
            }
        })
    }

    fn example_output(&self) -> String {
        "Container 'redis' restarted successfully".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let container = parameters
            .get("container")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: container"))?;
        let action = parameters
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: action"))?;
        let timeout = parameters
            .get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(config.docker_timeout);
        let docker_cmd = match action {
            "start" => "start",
            "stop" => "stop",
            "restart" => "restart",
            "pause" => "pause",
            "unpause" => "unpause",
            _ => return Err(anyhow::anyhow!("Unknown action: {}", action)),
        };
        let mut cmd = Command::new("docker");
        if config.is_docker_configured() {
            cmd.env("DOCKER_HOST", config.get_docker_host());
        }
        cmd.arg(docker_cmd);
        if action == "stop" {
            cmd.arg("-t").arg(timeout.to_string());
        }
        cmd.arg(container);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "Failed to {} container: {}",
                action,
                stderr
            ));
        }
        Ok(format!(
            "Container '{}' {}ed successfully",
            container, action
        ))
    }
}

/// A skill for viewing Docker container logs.
#[derive(Debug)]
pub struct DockerLogsSkill;

#[async_trait::async_trait]
impl Skill for DockerLogsSkill {
    fn name(&self) -> &str {
        "docker_logs"
    }

    fn description(&self) -> &str {
        "View logs from a Docker container"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to debug container issues, monitor output, or check error logs"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name or ID".to_string(),
                required: true,
                default: None,
                example: Some(json!("my_app")),
                enum_values: None,
            },
            SkillParameter {
                name: "tail".to_string(),
                param_type: "integer".to_string(),
                description: "Number of lines to show from the end".to_string(),
                required: false,
                default: Some(json!(100)),
                example: Some(json!(50)),
                enum_values: None,
            },
            SkillParameter {
                name: "since".to_string(),
                param_type: "string".to_string(),
                description: "Show logs since timestamp (e.g., '2024-01-01T00:00:00Z' or '1h')"
                    .to_string(),
                required: false,
                default: None,
                example: Some(json!("1h")),
                enum_values: None,
            },
            SkillParameter {
                name: "follow".to_string(),
                param_type: "boolean".to_string(),
                description: "Follow log output (default: false)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "timestamps".to_string(),
                param_type: "boolean".to_string(),
                description: "Show timestamps (default: false)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "docker_logs",
            "parameters": {
                "container": "mysql",
                "tail": 20
            }
        })
    }

    fn example_output(&self) -> String {
        "2024-01-15T10:30:00Z [Note] [MY-010914] [Server] Shutdown complete\n2024-01-15T10:30:01Z [System] [MY-010116] [Server] /usr/sbin/mysqld: ready for connections".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let container = parameters
            .get("container")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: container"))?;
        let tail = parameters
            .get("tail")
            .and_then(|v| v.as_u64())
            .unwrap_or(100);
        let since = parameters.get("since").and_then(|v| v.as_str());
        let follow = parameters
            .get("follow")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let timestamps = parameters
            .get("timestamps")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut cmd = Command::new("docker");
        if config.is_docker_configured() {
            cmd.env("DOCKER_HOST", config.get_docker_host());
        }
        cmd.arg("logs");
        cmd.arg("--tail").arg(tail.to_string());
        if let Some(s) = since {
            cmd.arg("--since").arg(s);
        }
        if follow {
            cmd.arg("--follow");
        }
        if timestamps {
            cmd.arg("--timestamps");
        }
        cmd.arg(container);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to get logs: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            Ok("No logs available".to_string())
        } else {
            Ok(stdout.to_string())
        }
    }
}

/// A skill for getting detailed information about a Docker container.
#[derive(Debug)]
pub struct DockerInspectSkill;

#[async_trait::async_trait]
impl Skill for DockerInspectSkill {
    fn name(&self) -> &str {
        "docker_inspect"
    }

    fn description(&self) -> &str {
        "Get detailed JSON information about a Docker container"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when you need detailed container configuration, network settings, or mount information"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name or ID".to_string(),
                required: true,
                default: None,
                example: Some(json!("my_container")),
                enum_values: None,
            },
            SkillParameter {
                name: "format".to_string(),
                param_type: "string".to_string(),
                description: "Go template format for output".to_string(),
                required: false,
                default: None,
                example: Some(json!("{{.Name}} {{.State.Status}}")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "docker_inspect",
            "parameters": {
                "container": "nginx"
            }
        })
    }

    fn example_output(&self) -> String {
        "Detailed JSON container configuration".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let container = parameters
            .get("container")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: container"))?;
        let format = parameters.get("format").and_then(|v| v.as_str());
        let mut cmd = Command::new("docker");
        if config.is_docker_configured() {
            cmd.env("DOCKER_HOST", config.get_docker_host());
        }
        cmd.arg("inspect");
        if let Some(f) = format {
            cmd.arg("--format").arg(f);
        }
        cmd.arg(container);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to inspect container: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for executing commands in a running Docker container.
#[derive(Debug)]
pub struct DockerExecSkill;

#[async_trait::async_trait]
impl Skill for DockerExecSkill {
    fn name(&self) -> &str {
        "docker_exec"
    }

    fn description(&self) -> &str {
        "Execute a command inside a running Docker container"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to run commands inside containers for debugging, maintenance, or automation"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name or ID".to_string(),
                required: true,
                default: None,
                example: Some(json!("my_app")),
                enum_values: None,
            },
            SkillParameter {
                name: "command".to_string(),
                param_type: "string".to_string(),
                description: "Command to execute".to_string(),
                required: true,
                default: None,
                example: Some(json!("ls -la")),
                enum_values: None,
            },
            SkillParameter {
                name: "interactive".to_string(),
                param_type: "boolean".to_string(),
                description: "Keep STDIN open (default: false)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "tty".to_string(),
                param_type: "boolean".to_string(),
                description: "Allocate a pseudo-TTY (default: false)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "workdir".to_string(),
                param_type: "string".to_string(),
                description: "Working directory inside the container".to_string(),
                required: false,
                default: None,
                example: Some(json!("/app")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "docker_exec",
            "parameters": {
                "container": "mysql",
                "command": "mysql -e 'SHOW DATABASES'"
            }
        })
    }

    fn example_output(&self) -> String {
        "Database\ninformation_schema\nmysql\nperformance_schema\nsys".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let container = parameters
            .get("container")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: container"))?;
        let command = parameters
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: command"))?;
        let interactive = parameters
            .get("interactive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let tty = parameters
            .get("tty")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let workdir = parameters.get("workdir").and_then(|v| v.as_str());
        let mut cmd = Command::new("docker");
        if config.is_docker_configured() {
            cmd.env("DOCKER_HOST", config.get_docker_host());
        }
        cmd.arg("exec");
        if interactive {
            cmd.arg("-i");
        }
        if tty {
            cmd.arg("-t");
        }
        if let Some(wd) = workdir {
            cmd.arg("-w").arg(wd);
        }
        cmd.arg(container);
        cmd.arg("sh");
        cmd.arg("-c");
        cmd.arg(command);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Command failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            Ok("Command executed successfully (no output)".to_string())
        } else {
            Ok(stdout.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_ps() {
        let skill = DockerPsSkill;
        let params = HashMap::new();
        let result = skill.execute(&params).await;
        if let Ok(output) = result {
            assert!(output.contains("CONTAINER") || output.is_empty());
        }
    }
}
