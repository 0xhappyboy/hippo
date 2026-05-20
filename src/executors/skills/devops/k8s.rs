//! k8s container orchestration utilities.
//!
//! This module provides skills for k8s operations:
//! - `K8sGetPodsSkill`: List pods in a namespace
//! - `K8sDescribePodSkill`: Get detailed pod information
//! - `K8sGetLogsSkill`: Get pod logs
//! - `K8sExecSkill`: Execute commands in a pod
//! - `K8sGetDeploymentsSkill`: List deployments
//! - `K8sGetServicesSkill`: List services
//! - `K8sScaleDeploymentSkill`: Scale a deployment
//! - `K8sRestartDeploymentSkill`: Restart a deployment
//! - `K8sPortForwardSkill`: Port forward to a pod
//! - `K8sGetNodesSkill`: List cluster nodes
//! - `K8sGetNamespacesSkill`: List namespaces
//! - `K8sApplyYamlSkill`: Apply YAML/JSON manifest
//! - `K8sDeleteResourceSkill`: Delete k8s resources
//! - `K8sGetEventsSkill`: Get cluster events
//! - `K8sGetConfigMapsSkill`: List configmaps
//! - `K8sGetSecretsSkill`: List secrets
//! - `K8sGetIngressesSkill`: List ingresses
//! - `K8sGetStatefulSetsSkill`: List statefulsets

use crate::config::get_config;
use crate::executors::types::{Skill, SkillParameter};
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process::Command;

/// A skill for listing k8s pods.
#[derive(Debug)]
pub struct K8sGetPodsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetPodsSkill {
    fn name(&self) -> &str {
        "k8s_get_pods"
    }

    fn description(&self) -> &str {
        "List k8s pods in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when you need to see running pods, check pod status, or find pod names in a k8s cluster"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("kube-system")),
                enum_values: None,
            },
            SkillParameter {
                name: "all_namespaces".to_string(),
                param_type: "boolean".to_string(),
                description: "List pods in all namespaces".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "selector".to_string(),
                param_type: "string".to_string(),
                description: "Label selector to filter pods (e.g., 'app=nginx')".to_string(),
                required: false,
                default: None,
                example: Some(json!("app=myapp")),
                enum_values: None,
            },
            SkillParameter {
                name: "output".to_string(),
                param_type: "string".to_string(),
                description: "Output format: wide, json, yaml".to_string(),
                required: false,
                default: Some(json!("wide")),
                example: Some(json!("json")),
                enum_values: Some(vec![
                    "wide".to_string(),
                    "json".to_string(),
                    "yaml".to_string(),
                ]),
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_pods",
            "parameters": {
                "namespace": "production",
                "selector": "app=web"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME                     READY   STATUS    RESTARTS   AGE   IP           NODE\nweb-7b4c8d9f6-abc12       1/1     Running   0          5d    10.244.1.2   node-1\nweb-7b4c8d9f6-def34       1/1     Running   0          5d    10.244.2.3   node-2".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let all_namespaces = parameters
            .get("all_namespaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let selector = parameters.get("selector").and_then(|v| v.as_str());
        let output = parameters
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("wide");
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("pods");
        if all_namespaces {
            cmd.arg("--all-namespaces");
        } else {
            cmd.arg("-n").arg(namespace);
        }
        if let Some(sel) = selector {
            cmd.arg("-l").arg(sel);
        }
        match output {
            "json" => {
                cmd.arg("-o").arg("json");
            }
            "yaml" => {
                cmd.arg("-o").arg("yaml");
            }
            _ => {
                cmd.arg("-o").arg("wide");
            }
        }
        let output_result = cmd.output()?;
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow::anyhow!("kubectl get pods failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        if output == "json" {
            if let Ok(json_value) = serde_json::from_str(&stdout).map(|v: serde_json::Value| v) {
                return Ok(serde_json::to_string_pretty(&json_value)?);
            }
        }
        Ok(stdout.to_string())
    }
}

/// A skill for getting detailed pod information.
#[derive(Debug)]
pub struct K8sDescribePodSkill;

#[async_trait::async_trait]
impl Skill for K8sDescribePodSkill {
    fn name(&self) -> &str {
        "k8s_describe_pod"
    }

    fn description(&self) -> &str {
        "Get detailed information about a k8s pod"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to debug pod issues, check pod events, or get detailed pod configuration"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "pod".to_string(),
                param_type: "string".to_string(),
                description: "Pod name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-pod-abc123")),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_describe_pod",
            "parameters": {
                "pod": "nginx-7b4c8d9f6-abc12",
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "Name:         nginx-7b4c8d9f6-abc12\nNamespace:    default\nPriority:     0\nNode:         node-1/192.168.1.10\n...".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let pod = parameters
            .get("pod")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: pod"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("describe")
            .arg("pod")
            .arg(pod)
            .arg("-n")
            .arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl describe failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for getting pod logs.
#[derive(Debug)]
pub struct K8sGetLogsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetLogsSkill {
    fn name(&self) -> &str {
        "k8s_get_logs"
    }

    fn description(&self) -> &str {
        "Get logs from a k8s pod"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to debug pod issues, check application logs, or monitor pod output"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "pod".to_string(),
                param_type: "string".to_string(),
                description: "Pod name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-app-abc123")),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name (for pods with multiple containers)".to_string(),
                required: false,
                default: None,
                example: Some(json!("app")),
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
                description: "Show logs since duration (e.g., '1h', '30m')".to_string(),
                required: false,
                default: None,
                example: Some(json!("1h")),
                enum_values: None,
            },
            SkillParameter {
                name: "previous".to_string(),
                param_type: "boolean".to_string(),
                description: "Get logs from previous container instance".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
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
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_logs",
            "parameters": {
                "pod": "nginx-7b4c8d9f6-abc12",
                "tail": 50
            }
        })
    }

    fn example_output(&self) -> String {
        "2024-01-15T10:30:00Z [info] Server started\n2024-01-15T10:30:01Z [info] Listening on port 80".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let pod = parameters
            .get("pod")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: pod"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let container = parameters.get("container").and_then(|v| v.as_str());
        let tail = parameters
            .get("tail")
            .and_then(|v| v.as_u64())
            .unwrap_or(100);
        let since = parameters.get("since").and_then(|v| v.as_str());
        let previous = parameters
            .get("previous")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let follow = parameters
            .get("follow")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("logs").arg(pod).arg("-n").arg(namespace);
        cmd.arg("--tail").arg(tail.to_string());
        if let Some(c) = container {
            cmd.arg("-c").arg(c);
        }
        if let Some(s) = since {
            cmd.arg("--since").arg(s);
        }
        if previous {
            cmd.arg("--previous");
        }
        if follow {
            cmd.arg("--follow");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl logs failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            Ok("No logs available".to_string())
        } else {
            Ok(stdout.to_string())
        }
    }
}

/// A skill for executing commands in a pod.
#[derive(Debug)]
pub struct K8sExecSkill;

#[async_trait::async_trait]
impl Skill for K8sExecSkill {
    fn name(&self) -> &str {
        "k8s_exec"
    }

    fn description(&self) -> &str {
        "Execute a command inside a k8s pod"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to run commands inside pods for debugging, maintenance, or diagnostics"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "pod".to_string(),
                param_type: "string".to_string(),
                description: "Pod name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-app-abc123")),
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
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
            SkillParameter {
                name: "container".to_string(),
                param_type: "string".to_string(),
                description: "Container name (for pods with multiple containers)".to_string(),
                required: false,
                default: None,
                example: Some(json!("app")),
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
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_exec",
            "parameters": {
                "pod": "mysql-abc123",
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
        let pod = parameters
            .get("pod")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: pod"))?;
        let command = parameters
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: command"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let container = parameters.get("container").and_then(|v| v.as_str());
        let interactive = parameters
            .get("interactive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let tty = parameters
            .get("tty")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("exec").arg(pod).arg("-n").arg(namespace);
        if interactive {
            cmd.arg("-i");
        }
        if tty {
            cmd.arg("-t");
        }
        if let Some(c) = container {
            cmd.arg("-c").arg(c);
        }
        cmd.arg("--").arg("sh").arg("-c").arg(command);
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

/// A skill for listing deployments.
#[derive(Debug)]
pub struct K8sGetDeploymentsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetDeploymentsSkill {
    fn name(&self) -> &str {
        "k8s_get_deployments"
    }

    fn description(&self) -> &str {
        "List k8s deployments in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to check deployment status, replicas, and rollout history"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
            SkillParameter {
                name: "all_namespaces".to_string(),
                param_type: "boolean".to_string(),
                description: "List deployments in all namespaces".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "output".to_string(),
                param_type: "string".to_string(),
                description: "Output format: wide, json, yaml".to_string(),
                required: false,
                default: Some(json!("wide")),
                example: Some(json!("json")),
                enum_values: Some(vec![
                    "wide".to_string(),
                    "json".to_string(),
                    "yaml".to_string(),
                ]),
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_deployments",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME    READY   UP-TO-DATE   AVAILABLE   AGE\nnginx   3/3     3            3           5d"
            .to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let all_namespaces = parameters
            .get("all_namespaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let output = parameters
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("wide");
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("deployments");

        if all_namespaces {
            cmd.arg("--all-namespaces");
        } else {
            cmd.arg("-n").arg(namespace);
        }
        match output {
            "json" => cmd.arg("-o").arg("json"),
            "yaml" => cmd.arg("-o").arg("yaml"),
            _ => cmd.arg("-o").arg("wide"),
        };
        let output_result = cmd.output()?;
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow::anyhow!(
                "kubectl get deployments failed: {}",
                stderr
            ));
        }
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        if output == "json" {
            if let Ok(json_value) = serde_json::from_str(&stdout).map(|v: serde_json::Value| v) {
                return Ok(serde_json::to_string_pretty(&json_value)?);
            }
        }
        Ok(stdout.to_string())
    }
}

/// A skill for listing services.
#[derive(Debug)]
pub struct K8sGetServicesSkill;

#[async_trait::async_trait]
impl Skill for K8sGetServicesSkill {
    fn name(&self) -> &str {
        "k8s_get_services"
    }

    fn description(&self) -> &str {
        "List k8s services in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to check service endpoints, ClusterIPs, and port mappings"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
            SkillParameter {
                name: "all_namespaces".to_string(),
                param_type: "boolean".to_string(),
                description: "List services in all namespaces".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "output".to_string(),
                param_type: "string".to_string(),
                description: "Output format: wide, json, yaml".to_string(),
                required: false,
                default: Some(json!("wide")),
                example: Some(json!("json")),
                enum_values: Some(vec![
                    "wide".to_string(),
                    "json".to_string(),
                    "yaml".to_string(),
                ]),
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_services",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME         TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)        AGE\nk8s   ClusterIP   10.96.0.1       <none>        443/TCP        10d\nnginx        LoadBalancer 10.96.100.50   192.168.1.10  80:30080/TCP   5d".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let all_namespaces = parameters
            .get("all_namespaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let output = parameters
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("wide");
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("services");
        if all_namespaces {
            cmd.arg("--all-namespaces");
        } else {
            cmd.arg("-n").arg(namespace);
        }
        match output {
            "json" => cmd.arg("-o").arg("json"),
            "yaml" => cmd.arg("-o").arg("yaml"),
            _ => cmd.arg("-o").arg("wide"),
        };
        let output_result = cmd.output()?;
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow::anyhow!("kubectl get services failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        if output == "json" {
            if let Ok(json_value) = serde_json::from_str(&stdout).map(|v: serde_json::Value| v) {
                return Ok(serde_json::to_string_pretty(&json_value)?);
            }
        }
        Ok(stdout.to_string())
    }
}

/// A skill for scaling a deployment.
#[derive(Debug)]
pub struct K8sScaleDeploymentSkill;

#[async_trait::async_trait]
impl Skill for K8sScaleDeploymentSkill {
    fn name(&self) -> &str {
        "k8s_scale_deployment"
    }

    fn description(&self) -> &str {
        "Scale a k8s deployment to the desired number of replicas"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to scale applications up or down based on load"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "deployment".to_string(),
                param_type: "string".to_string(),
                description: "Deployment name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-app")),
                enum_values: None,
            },
            SkillParameter {
                name: "replicas".to_string(),
                param_type: "integer".to_string(),
                description: "Number of replicas".to_string(),
                required: true,
                default: None,
                example: Some(json!(3)),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_scale_deployment",
            "parameters": {
                "deployment": "nginx",
                "replicas": 5
            }
        })
    }

    fn example_output(&self) -> String {
        "Deployment 'nginx' scaled to 5 replicas".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let deployment = parameters
            .get("deployment")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: deployment"))?;
        let replicas = parameters
            .get("replicas")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: replicas"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("scale")
            .arg("deployment")
            .arg(deployment)
            .arg("--replicas")
            .arg(replicas.to_string())
            .arg("-n")
            .arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Scale failed: {}", stderr));
        }
        Ok(format!(
            "Deployment '{}' scaled to {} replicas",
            deployment, replicas
        ))
    }
}

/// A skill for restarting a deployment.
#[derive(Debug)]
pub struct K8sRestartDeploymentSkill;

#[async_trait::async_trait]
impl Skill for K8sRestartDeploymentSkill {
    fn name(&self) -> &str {
        "k8s_restart_deployment"
    }

    fn description(&self) -> &str {
        "Restart a k8s deployment by rolling restart"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to restart applications after config changes or to recover from issues"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "deployment".to_string(),
                param_type: "string".to_string(),
                description: "Deployment name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-app")),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_restart_deployment",
            "parameters": {
                "deployment": "nginx"
            }
        })
    }

    fn example_output(&self) -> String {
        "Deployment 'nginx' restarted successfully".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let deployment = parameters
            .get("deployment")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: deployment"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("rollout")
            .arg("restart")
            .arg("deployment")
            .arg(deployment)
            .arg("-n")
            .arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Restart failed: {}", stderr));
        }
        Ok(format!(
            "Deployment '{}' restarted successfully",
            deployment
        ))
    }
}

/// A skill for port forwarding to a pod.
#[derive(Debug)]
pub struct K8sPortForwardSkill;

#[async_trait::async_trait]
impl Skill for K8sPortForwardSkill {
    fn name(&self) -> &str {
        "k8s_port_forward"
    }

    fn description(&self) -> &str {
        "Forward a local port to a port on a k8s pod"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to access pod services locally for debugging or testing"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "pod".to_string(),
                param_type: "string".to_string(),
                description: "Pod name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-app-abc123")),
                enum_values: None,
            },
            SkillParameter {
                name: "local_port".to_string(),
                param_type: "integer".to_string(),
                description: "Local port to forward from".to_string(),
                required: true,
                default: None,
                example: Some(json!(8080)),
                enum_values: None,
            },
            SkillParameter {
                name: "remote_port".to_string(),
                param_type: "integer".to_string(),
                description: "Remote port on the pod".to_string(),
                required: true,
                default: None,
                example: Some(json!(80)),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_port_forward",
            "parameters": {
                "pod": "nginx-abc123",
                "local_port": 8080,
                "remote_port": 80
            }
        })
    }

    fn example_output(&self) -> String {
        "Port forwarding established: localhost:8080 -> pod:80".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let pod = parameters
            .get("pod")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: pod"))?;
        let local_port = parameters
            .get("local_port")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: local_port"))?;
        let remote_port = parameters
            .get("remote_port")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: remote_port"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("port-forward")
            .arg(pod)
            .arg(format!("{}:{}", local_port, remote_port))
            .arg("-n")
            .arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Port forward failed: {}", stderr));
        }
        Ok(format!(
            "Port forwarding established: localhost:{} -> pod:{}",
            local_port, remote_port
        ))
    }
}

/// A skill for listing cluster nodes.
#[derive(Debug)]
pub struct K8sGetNodesSkill;

#[async_trait::async_trait]
impl Skill for K8sGetNodesSkill {
    fn name(&self) -> &str {
        "k8s_get_nodes"
    }

    fn description(&self) -> &str {
        "List k8s cluster nodes and their status"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to check node health, capacity, and resource utilization"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "output".to_string(),
            param_type: "string".to_string(),
            description: "Output format: wide, json, yaml".to_string(),
            required: false,
            default: Some(json!("wide")),
            example: Some(json!("json")),
            enum_values: Some(vec![
                "wide".to_string(),
                "json".to_string(),
                "yaml".to_string(),
            ]),
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_nodes"
        })
    }

    fn example_output(&self) -> String {
        "NAME     STATUS   ROLES    AGE   VERSION   INTERNAL-IP   EXTERNAL-IP\nnode-1   Ready    master   10d   v1.28.0   192.168.1.10   <none>\nnode-2   Ready    worker   10d   v1.28.0   192.168.1.11   <none>".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let output = parameters
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("wide");
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("nodes");
        match output {
            "json" => cmd.arg("-o").arg("json"),
            "yaml" => cmd.arg("-o").arg("yaml"),
            _ => cmd.arg("-o").arg("wide"),
        };
        let output_result = cmd.output()?;
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow::anyhow!("kubectl get nodes failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        if output == "json" {
            if let Ok(json_value) = serde_json::from_str(&stdout).map(|v: serde_json::Value| v) {
                return Ok(serde_json::to_string_pretty(&json_value)?);
            }
        }
        Ok(stdout.to_string())
    }
}

/// A skill for listing namespaces.
#[derive(Debug)]
pub struct K8sGetNamespacesSkill;

#[async_trait::async_trait]
impl Skill for K8sGetNamespacesSkill {
    fn name(&self) -> &str {
        "k8s_get_namespaces"
    }

    fn description(&self) -> &str {
        "List k8s namespaces"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to see available namespaces and their status"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "output".to_string(),
            param_type: "string".to_string(),
            description: "Output format: json, yaml".to_string(),
            required: false,
            default: Some(json!("table")),
            example: Some(json!("json")),
            enum_values: Some(vec![
                "table".to_string(),
                "json".to_string(),
                "yaml".to_string(),
            ]),
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_namespaces"
        })
    }

    fn example_output(&self) -> String {
        "NAME              STATUS   AGE\ndefault           Active   10d\nkube-system       Active   10d\nkube-public       Active   10d".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let output = parameters
            .get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("table");
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("namespaces");
        match output {
            "json" => {
                cmd.arg("-o").arg("json");
            }
            "yaml" => {
                cmd.arg("-o").arg("yaml");
            }
            _ => {}
        }
        let output_result = cmd.output()?;
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow::anyhow!("kubectl get namespaces failed: {}", stderr));
        }
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        if output == "json" {
            if let Ok(json_value) = serde_json::from_str(&stdout).map(|v: serde_json::Value| v) {
                return Ok(serde_json::to_string_pretty(&json_value)?);
            }
        }
        Ok(stdout.to_string())
    }
}

/// A skill for applying YAML/JSON manifests.
#[derive(Debug)]
pub struct K8sApplyYamlSkill;

#[async_trait::async_trait]
impl Skill for K8sApplyYamlSkill {
    fn name(&self) -> &str {
        "k8s_apply_yaml"
    }

    fn description(&self) -> &str {
        "Apply a k8s YAML or JSON manifest"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to create or update k8s resources from manifests"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "manifest".to_string(),
                param_type: "string".to_string(),
                description: "YAML or JSON manifest content".to_string(),
                required: true,
                default: None,
                example: Some(json!(
                    "apiVersion: v1\nkind: Pod\nmetadata:\n  name: my-pod"
                )),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_apply_yaml",
            "parameters": {
                "manifest": "apiVersion: v1\nkind: Pod\nmetadata:\n  name: nginx\nspec:\n  containers:\n  - name: nginx\n    image: nginx:latest"
            }
        })
    }

    fn example_output(&self) -> String {
        "pod/nginx created".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let manifest = parameters
            .get("manifest")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: manifest"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("apply").arg("-f").arg("-").arg("-n").arg(namespace);
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(manifest.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Apply failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for deleting k8s resources.
#[derive(Debug)]
pub struct K8sDeleteResourceSkill;

#[async_trait::async_trait]
impl Skill for K8sDeleteResourceSkill {
    fn name(&self) -> &str {
        "k8s_delete_resource"
    }

    fn description(&self) -> &str {
        "Delete a k8s resource (pod, deployment, service, etc.)"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to remove unwanted resources from the cluster"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "resource_type".to_string(),
                param_type: "string".to_string(),
                description: "Resource type (pod, deployment, service, configmap, secret, etc.)"
                    .to_string(),
                required: true,
                default: None,
                example: Some(json!("pod")),
                enum_values: Some(vec![
                    "pod".to_string(),
                    "deployment".to_string(),
                    "service".to_string(),
                    "configmap".to_string(),
                    "secret".to_string(),
                    "ingress".to_string(),
                    "statefulset".to_string(),
                    "daemonset".to_string(),
                ]),
            },
            SkillParameter {
                name: "name".to_string(),
                param_type: "string".to_string(),
                description: "Resource name".to_string(),
                required: true,
                default: None,
                example: Some(json!("my-pod")),
                enum_values: None,
            },
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("production")),
                enum_values: None,
            },
            SkillParameter {
                name: "force".to_string(),
                param_type: "boolean".to_string(),
                description: "Force delete (for pods)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_delete_resource",
            "parameters": {
                "resource_type": "deployment",
                "name": "nginx"
            }
        })
    }

    fn example_output(&self) -> String {
        "deployment.apps/nginx deleted".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let resource_type = parameters
            .get("resource_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: resource_type"))?;
        let name = parameters
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: name"))?;
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let force = parameters
            .get("force")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("delete")
            .arg(resource_type)
            .arg(name)
            .arg("-n")
            .arg(namespace);
        if force && resource_type == "pod" {
            cmd.arg("--force").arg("--grace-period=0");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Delete failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for getting cluster events.
#[derive(Debug)]
pub struct K8sGetEventsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetEventsSkill {
    fn name(&self) -> &str {
        "k8s_get_events"
    }

    fn description(&self) -> &str {
        "Get k8s cluster events"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to debug issues, see what's happening in the cluster, or monitor changes"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "namespace".to_string(),
                param_type: "string".to_string(),
                description: "k8s namespace (default: default)".to_string(),
                required: false,
                default: Some(json!("default")),
                example: Some(json!("kube-system")),
                enum_values: None,
            },
            SkillParameter {
                name: "all_namespaces".to_string(),
                param_type: "boolean".to_string(),
                description: "Get events from all namespaces".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
            SkillParameter {
                name: "watch".to_string(),
                param_type: "boolean".to_string(),
                description: "Watch events (default: false)".to_string(),
                required: false,
                default: Some(json!(false)),
                example: Some(json!(true)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_events",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "LAST SEEN   TYPE     REASON              OBJECT              MESSAGE\n2m          Normal   Scheduled           pod/my-pod          Successfully assigned default/my-pod to node-1".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let all_namespaces = parameters
            .get("all_namespaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let watch = parameters
            .get("watch")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("events");
        if all_namespaces {
            cmd.arg("--all-namespaces");
        } else {
            cmd.arg("-n").arg(namespace);
        }
        if watch {
            cmd.arg("--watch");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl get events failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for listing ConfigMaps.
#[derive(Debug)]
pub struct K8sGetConfigMapsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetConfigMapsSkill {
    fn name(&self) -> &str {
        "k8s_get_configmaps"
    }

    fn description(&self) -> &str {
        "List k8s ConfigMaps in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to see available ConfigMaps and their data"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "namespace".to_string(),
            param_type: "string".to_string(),
            description: "k8s namespace (default: default)".to_string(),
            required: false,
            default: Some(json!("default")),
            example: Some(json!("production")),
            enum_values: None,
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_configmaps",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME               DATA   AGE\napp-config         3      5d\ndb-config          2      5d"
            .to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("configmaps").arg("-n").arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl get configmaps failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for listing Secrets.
#[derive(Debug)]
pub struct K8sGetSecretsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetSecretsSkill {
    fn name(&self) -> &str {
        "k8s_get_secrets"
    }

    fn description(&self) -> &str {
        "List k8s Secrets in a namespace (names only, not values)"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to see available secrets (names only, not values)"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "namespace".to_string(),
            param_type: "string".to_string(),
            description: "k8s namespace (default: default)".to_string(),
            required: false,
            default: Some(json!("default")),
            example: Some(json!("production")),
            enum_values: None,
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_secrets",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME                  TYPE                                  DATA   AGE\ndb-secret             Opaque                                2      5d\ntls-secret            k8s.io/tls                     2      5d".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("secrets").arg("-n").arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl get secrets failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for listing Ingresses.
#[derive(Debug)]
pub struct K8sGetIngressesSkill;

#[async_trait::async_trait]
impl Skill for K8sGetIngressesSkill {
    fn name(&self) -> &str {
        "k8s_get_ingresses"
    }

    fn description(&self) -> &str {
        "List k8s Ingress resources in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to check ingress rules and external access configuration"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "namespace".to_string(),
            param_type: "string".to_string(),
            description: "k8s namespace (default: default)".to_string(),
            required: false,
            default: Some(json!("default")),
            example: Some(json!("production")),
            enum_values: None,
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_ingresses",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME         CLASS    HOSTS         ADDRESS        PORTS   AGE\nmy-ingress   nginx    example.com   192.168.1.10   80      5d".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("ingresses").arg("-n").arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("kubectl get ingresses failed: {}", stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A skill for listing StatefulSets.
#[derive(Debug)]
pub struct K8sGetStatefulSetsSkill;

#[async_trait::async_trait]
impl Skill for K8sGetStatefulSetsSkill {
    fn name(&self) -> &str {
        "k8s_get_statefulsets"
    }

    fn description(&self) -> &str {
        "List k8s StatefulSets in a namespace"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill to check stateful applications like databases"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![SkillParameter {
            name: "namespace".to_string(),
            param_type: "string".to_string(),
            description: "k8s namespace (default: default)".to_string(),
            required: false,
            default: Some(json!("default")),
            example: Some(json!("production")),
            enum_values: None,
        }]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "k8s_get_statefulsets",
            "parameters": {
                "namespace": "default"
            }
        })
    }

    fn example_output(&self) -> String {
        "NAME     READY   AGE\nmysql    3/3     10d".to_string()
    }

    fn category(&self) -> &str {
        "devops"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let config = get_config();
        let namespace = parameters
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or(&config.k8s_namespace);
        let mut cmd = config.build_kubectl_command();
        cmd.arg("get").arg("statefulsets").arg("-n").arg(namespace);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "kubectl get statefulsets failed: {}",
                stderr
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_k8s_get_namespaces() {
        let skill = K8sGetNamespacesSkill;
        let params = HashMap::new();
        let result = skill.execute(&params).await;
        if let Ok(output) = result {
            assert!(output.contains("default") || output.contains("NAME"));
        }
    }
}
