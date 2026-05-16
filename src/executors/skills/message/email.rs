use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    config,
    executors::types::{Skill, SkillParameter},
};

#[derive(Debug)]
pub struct SendEmailSkill;

#[async_trait::async_trait]
impl Skill for SendEmailSkill {
    fn name(&self) -> &str {
        "send_email"
    }

    fn description(&self) -> &str {
        "Send an email via SMTP server"
    }

    fn usage_hint(&self) -> &str {
        "Use this skill when the user wants to send an email, notify someone via email, or send a message to an email address"
    }

    fn parameters(&self) -> Vec<SkillParameter> {
        vec![
            SkillParameter {
                name: "to".to_string(),
                param_type: "string".to_string(),
                description: "Recipient email address".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("user@example.com".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "subject".to_string(),
                param_type: "string".to_string(),
                description: "Email subject line".to_string(),
                required: true,
                default: None,
                example: Some(Value::String("Hello from Hippo".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "body".to_string(),
                param_type: "string".to_string(),
                description: "Email body content (supports HTML)".to_string(),
                required: true,
                default: None,
                example: Some(Value::String(
                    "<h1>Hello</h1><p>This is a test email.</p>".to_string(),
                )),
                enum_values: None,
            },
            SkillParameter {
                name: "from".to_string(),
                param_type: "string".to_string(),
                description: "Sender email address (overrides default)".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("bot@example.com".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "cc".to_string(),
                param_type: "string".to_string(),
                description: "CC recipient email address".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("cc@example.com".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "bcc".to_string(),
                param_type: "string".to_string(),
                description: "BCC recipient email address".to_string(),
                required: false,
                default: None,
                example: Some(Value::String("bcc@example.com".to_string())),
                enum_values: None,
            },
            SkillParameter {
                name: "is_html".to_string(),
                param_type: "boolean".to_string(),
                description: "Whether the body is HTML (default: true)".to_string(),
                required: false,
                default: Some(Value::Bool(true)),
                example: Some(Value::Bool(false)),
                enum_values: None,
            },
        ]
    }

    fn example_call(&self) -> Value {
        json!({
            "action": "send_email",
            "parameters": {
                "to": "user@example.com",
                "subject": "Hello",
                "body": "This is a test email"
            }
        })
    }

    fn example_output(&self) -> String {
        "Email sent successfully to user@example.com".to_string()
    }

    fn category(&self) -> &str {
        "messaging"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let to = parameters
            .get("to")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'to' parameter"))?;
        let subject = parameters
            .get("subject")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'subject' parameter"))?;
        let body = parameters
            .get("body")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'body' parameter"))?;
        let from_override = parameters
            .get("from")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let cc = parameters.get("cc").and_then(|v| v.as_str());
        let bcc = parameters.get("bcc").and_then(|v| v.as_str());
        let is_html = parameters
            .get("is_html")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let config = config::get_config();
        if !config.is_smtp_configured() {
            anyhow::bail!(
                "SMTP not configured. Please set param: smtp_host, smtp_port, smtp_username, smtp_password, smtp_from"
            );
        }
        let smtp_host = config.smtp_host;
        let smtp_port = config.smtp_port;
        let smtp_username = config.smtp_username;
        let smtp_password = config.smtp_password;
        let smtp_from = if from_override.is_empty() {
            config.smtp_from
        } else {
            from_override.to_string()
        };
        use lettre::message::MultiPart;
        use lettre::{
            AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
            transport::smtp::authentication::Credentials,
        };
        let to_addr: Mailbox = to.parse()?;
        let from_addr: Mailbox = smtp_from.parse()?;
        let mut email_builder = Message::builder()
            .from(from_addr)
            .to(to_addr)
            .subject(subject);
        if let Some(cc_addr) = cc {
            let cc_parsed: Mailbox = cc_addr.parse()?;
            email_builder = email_builder.cc(cc_parsed);
        }
        if let Some(bcc_addr) = bcc {
            let bcc_parsed: Mailbox = bcc_addr.parse()?;
            email_builder = email_builder.bcc(bcc_parsed);
        }
        let email = if is_html {
            email_builder.multipart(MultiPart::alternative_plain_html(
                String::new(),
                body.to_string(),
            ))?
        } else {
            email_builder.body(body.to_string())?
        };
        let creds = Credentials::new(smtp_username, smtp_password);
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();
        match mailer.send(email).await {
            Ok(_) => Ok(format!("Email sent successfully to {}", to)),
            Err(e) => Err(anyhow::anyhow!("Failed to send email: {}", e)),
        }
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("to")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: to"))?;
        parameters
            .get("subject")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: subject"))?;
        parameters
            .get("body")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: body"))?;
        Ok(())
    }
}
