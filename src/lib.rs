mod config;
mod core;
mod envs;
mod executors;
mod global;
mod i18n;
mod protocols;
mod skill_loader;
mod skill_scheduler;
mod types;

pub use config::{GLOBAL_CONFIG, HippoxConfig, get_config, init_config_from_env};
pub use core::{Hippox, ServiceConfig};
pub use langhub::types::ModelProvider;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::init_config_from_json_file;
    use crate::config::init_config_from_params;
    use crate::config::init_config_from_params_json_str;
    use crate::config::init_config_from_toml_file;
    use std::fs;
    use tempfile::NamedTempFile;
    use tokio;

    #[tokio::test]
    async fn test_main_logic() -> anyhow::Result<()> {
        tracing_subscriber::fmt().init();
        i18n::init();
        init_config_from_env();
        let lang = get_config().lang.clone();
        let provider = match get_config().provider.as_str() {
            "deepseek" => ModelProvider::DeepSeek,
            "anthropic" => ModelProvider::Anthropic,
            "google" => ModelProvider::Google,
            _ => ModelProvider::OpenAI,
        };
        let hippox = Hippox::new("skills", provider, &lang).await?;
        let config = ServiceConfig {
            enable_cli: get_config().enable_cli,
            enable_tcp: get_config().enable_tcp,
            enable_http: get_config().enable_http,
            enable_websocket: get_config().enable_websocket,
            enable_grpc: false,
        };
        hippox.start(config).await?;
        Ok(())
    }

    #[test]
    fn test_init_config_from_params() {
        init_config_from_params(
            Some("zh".to_string()),
            Some("deepseek".to_string()),
            Some(false),
            Some(true),
            Some(false),
            Some(true),
            Some(false),
            Some("smtp.example.com".to_string()),
            Some(465),
            Some("user@example.com".to_string()),
            Some("password".to_string()),
            Some("sender@example.com".to_string()),
            Some("123456:ABC".to_string()),
            Some("dingtalk_token".to_string()),
        );
        let config = get_config();
        assert_eq!(config.lang, "zh");
        assert_eq!(config.provider, "deepseek");
        assert_eq!(config.enable_cli, false);
        assert_eq!(config.enable_tcp, true);
        assert_eq!(config.enable_http, false);
        assert_eq!(config.enable_websocket, true);
        assert_eq!(config.enable_grpc, false);
        assert_eq!(config.smtp_host, "smtp.example.com");
        assert_eq!(config.smtp_port, 465);
        assert_eq!(config.smtp_username, "user@example.com");
        assert_eq!(config.smtp_password, "password");
        assert_eq!(config.smtp_from, "sender@example.com");
        assert_eq!(config.telegram_bot_token, "123456:ABC");
        assert_eq!(config.dingtalk_access_token, "dingtalk_token");
    }

    #[test]
    fn test_init_config_from_params_json_str() -> anyhow::Result<()> {
        let json_str = r#"{
            "lang": "zh",
            "provider": "anthropic",
            "enable_cli": false,
            "enable_http": true,
            "smtp_host": "smtp.gmail.com",
            "telegram_bot_token": "789xyz"
        }"#;
        init_config_from_params_json_str(json_str)?;
        let config = get_config();
        assert_eq!(config.lang, "zh");
        assert_eq!(config.provider, "anthropic");
        assert_eq!(config.enable_cli, false);
        assert_eq!(config.enable_http, true);
        assert_eq!(config.smtp_host, "smtp.gmail.com");
        assert_eq!(config.telegram_bot_token, "789xyz");
        assert_eq!(config.enable_tcp, false);
        assert_eq!(config.enable_websocket, false);
        Ok(())
    }

    #[test]
    fn test_init_config_from_toml_file() -> anyhow::Result<()> {
        let toml_content = r#"
lang = "zh"
provider = "google"
enable_cli = false
enable_http = true
smtp_host = "smtp.qq.com"
smtp_port = 587
telegram_bot_token = "test_token_123"
"#;
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_str().unwrap();
        fs::write(temp_file.path(), toml_content)?;
        init_config_from_toml_file(path)?;
        let config = get_config();
        assert_eq!(config.lang, "zh");
        assert_eq!(config.provider, "google");
        assert_eq!(config.enable_cli, false);
        assert_eq!(config.enable_http, true);
        assert_eq!(config.smtp_host, "smtp.qq.com");
        assert_eq!(config.smtp_port, 587);
        assert_eq!(config.telegram_bot_token, "test_token_123");
        Ok(())
    }

    #[test]
    fn test_init_config_from_json_file() -> anyhow::Result<()> {
        let json_content = r#"{
            "lang": "zh",
            "provider": "deepseek",
            "enable_cli": false,
            "enable_websocket": true,
            "smtp_host": "smtp.163.com",
            "smtp_port": 465,
            "telegram_bot_token": "json_token_456"
        }"#;
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_str().unwrap();
        fs::write(temp_file.path(), json_content)?;
        init_config_from_json_file(path)?;
        let config = get_config();
        assert_eq!(config.lang, "zh");
        assert_eq!(config.provider, "deepseek");
        assert_eq!(config.enable_cli, false);
        assert_eq!(config.enable_websocket, true);
        assert_eq!(config.smtp_host, "smtp.163.com");
        assert_eq!(config.smtp_port, 465);
        assert_eq!(config.telegram_bot_token, "json_token_456");
        Ok(())
    }

    #[test]
    fn test_config_methods() -> anyhow::Result<()> {
        init_config_from_params(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("smtp.test.com".to_string()),
            Some(587),
            Some("user".to_string()),
            Some("pass".to_string()),
            Some("from@test.com".to_string()),
            Some("telegram_token".to_string()),
            Some("dingtalk_token".to_string()),
        );
        let config = get_config();
        assert!(config.is_smtp_configured());
        assert!(config.is_telegram_configured());
        assert!(config.is_dingtalk_configured());
        init_config_from_params(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        );
        let config = get_config();
        assert!(!config.is_smtp_configured());
        assert!(!config.is_telegram_configured());
        assert!(!config.is_dingtalk_configured());
        Ok(())
    }

    #[test]
    fn test_load_from_params_method() -> anyhow::Result<()> {
        let config = HippoxConfig::load_from_params(
            Some("fr".to_string()),
            None,
            Some(false),
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("custom_token".to_string()),
            None,
        );
        assert_eq!(config.lang, "fr");
        assert_eq!(config.enable_cli, false);
        assert_eq!(config.enable_http, true);
        assert_eq!(config.telegram_bot_token, "custom_token");
        assert_eq!(config.provider, "openai");
        assert_eq!(config.enable_tcp, false);
        Ok(())
    }

    #[test]
    fn test_load_from_params_json_str_method() -> anyhow::Result<()> {
        let json_str = r#"{"lang": "de", "smtp_port": 25, "enable_http": true}"#;
        let config = HippoxConfig::load_from_params_json_str(json_str)?;
        assert_eq!(config.lang, "de");
        assert_eq!(config.smtp_port, 25);
        assert_eq!(config.enable_http, true);
        Ok(())
    }
}
