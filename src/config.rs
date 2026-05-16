use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::envs;

/// Hippox global configuration
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HippoxConfig {
    // Application settings
    pub lang: String,
    pub provider: String,
    // Service settings
    pub enable_cli: bool,
    pub enable_tcp: bool,
    pub enable_http: bool,
    pub enable_websocket: bool,
    pub enable_grpc: bool,
    // SMTP settings
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    // Telegram settings
    pub telegram_bot_token: String,
    // DingTalk settings
    pub dingding_access_token: String,
    // Feishu settings
    pub feishu_webhook: String,
    // WeCom settings
    pub wecom_webhook: String,
}

impl Default for HippoxConfig {
    fn default() -> Self {
        Self {
            lang: "en".to_string(),
            provider: "openai".to_string(),
            enable_cli: true,
            enable_tcp: false,
            enable_http: false,
            enable_websocket: false,
            enable_grpc: false,
            smtp_host: String::new(),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            smtp_from: String::new(),
            telegram_bot_token: String::new(),
            dingding_access_token: String::new(),
            feishu_webhook: String::new(),
            wecom_webhook: String::new(),
        }
    }
}

impl HippoxConfig {
    /// Load configuration from environment variables
    pub fn load_from_env() -> Self {
        Self {
            lang: envs::get_env_or(envs::HIPPOX_LANG, "en"),
            provider: envs::get_env_or(envs::HIPPOX_PROVIDER, "openai"),
            enable_cli: envs::is_env_true(envs::HIPPOX_ENABLE_CLI),
            enable_tcp: envs::is_env_true(envs::HIPPOX_ENABLE_TCP),
            enable_http: envs::is_env_true(envs::HIPPOX_ENABLE_HTTP),
            enable_websocket: envs::is_env_true(envs::HIPPOX_ENABLE_WS),
            enable_grpc: false,
            smtp_host: envs::get_env_or(envs::HIPPOX_SMTP_HOST, ""),
            smtp_port: envs::get_env_or(envs::HIPPOX_SMTP_PORT, "587")
                .parse::<u16>()
                .unwrap_or(587),
            smtp_username: envs::get_env_or(envs::HIPPOX_SMTP_USERNAME, ""),
            smtp_password: envs::get_env_or(envs::HIPPOX_SMTP_PASSWORD, ""),
            smtp_from: envs::get_env_or(envs::HIPPOX_SMTP_FROM, ""),
            telegram_bot_token: envs::get_env_or(envs::HIPPOX_TELEGRAM_BOT_TOKEN, ""),
            dingding_access_token: envs::get_env_or(envs::HIPPOX_DINGDING_ACCESS_TOKEN, ""),
            feishu_webhook: envs::get_env_or(envs::HIPPOX_FEISHU_WEBHOOK, ""),
            wecom_webhook: envs::get_env_or(envs::HIPPOX_WECOM_WEBHOOK, ""),
        }
    }

    /// Load from TOML configuration file
    pub fn load_from_toml_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: HippoxConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load from JSON configuration file
    pub fn load_from_json_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: HippoxConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Load from optional parameters, only set fields that are Some
    #[allow(clippy::too_many_arguments)]
    pub fn load_from_params(
        lang: Option<String>,
        provider: Option<String>,
        enable_cli: Option<bool>,
        enable_tcp: Option<bool>,
        enable_http: Option<bool>,
        enable_websocket: Option<bool>,
        enable_grpc: Option<bool>,
        smtp_host: Option<String>,
        smtp_port: Option<u16>,
        smtp_username: Option<String>,
        smtp_password: Option<String>,
        smtp_from: Option<String>,
        telegram_bot_token: Option<String>,
        dingding_access_token: Option<String>,
        feishu_webhook: Option<String>,
        wecom_webhook: Option<String>,
    ) -> Self {
        let mut config = Self::load_from_env();
        if let Some(v) = lang {
            config.lang = v;
        }
        if let Some(v) = provider {
            config.provider = v;
        }
        if let Some(v) = enable_cli {
            config.enable_cli = v;
        }
        if let Some(v) = enable_tcp {
            config.enable_tcp = v;
        }
        if let Some(v) = enable_http {
            config.enable_http = v;
        }
        if let Some(v) = enable_websocket {
            config.enable_websocket = v;
        }
        if let Some(v) = enable_grpc {
            config.enable_grpc = v;
        }
        if let Some(v) = smtp_host {
            config.smtp_host = v;
        }
        if let Some(v) = smtp_port {
            config.smtp_port = v;
        }
        if let Some(v) = smtp_username {
            config.smtp_username = v;
        }
        if let Some(v) = smtp_password {
            config.smtp_password = v;
        }
        if let Some(v) = smtp_from {
            config.smtp_from = v;
        }
        if let Some(v) = telegram_bot_token {
            config.telegram_bot_token = v;
        }
        if let Some(v) = dingding_access_token {
            config.dingding_access_token = v;
        }
        if let Some(v) = feishu_webhook {
            config.feishu_webhook = v;
        }
        if let Some(v) = wecom_webhook {
            config.wecom_webhook = v;
        }
        config
    }

    /// Load from JSON string of optional parameters, only set fields that are present
    pub fn load_from_params_json_str(json_str: &str) -> anyhow::Result<Self> {
        let overrides: serde_json::Value = serde_json::from_str(json_str)?;
        let mut config = Self::load_from_env();

        if let Some(v) = overrides.get("lang").and_then(|x| x.as_str()) {
            config.lang = v.to_string();
        }
        if let Some(v) = overrides.get("provider").and_then(|x| x.as_str()) {
            config.provider = v.to_string();
        }
        if let Some(v) = overrides.get("enable_cli").and_then(|x| x.as_bool()) {
            config.enable_cli = v;
        }
        if let Some(v) = overrides.get("enable_tcp").and_then(|x| x.as_bool()) {
            config.enable_tcp = v;
        }
        if let Some(v) = overrides.get("enable_http").and_then(|x| x.as_bool()) {
            config.enable_http = v;
        }
        if let Some(v) = overrides.get("enable_websocket").and_then(|x| x.as_bool()) {
            config.enable_websocket = v;
        }
        if let Some(v) = overrides.get("enable_grpc").and_then(|x| x.as_bool()) {
            config.enable_grpc = v;
        }
        if let Some(v) = overrides.get("smtp_host").and_then(|x| x.as_str()) {
            config.smtp_host = v.to_string();
        }
        if let Some(v) = overrides.get("smtp_port").and_then(|x| x.as_u64()) {
            config.smtp_port = v as u16;
        }
        if let Some(v) = overrides.get("smtp_username").and_then(|x| x.as_str()) {
            config.smtp_username = v.to_string();
        }
        if let Some(v) = overrides.get("smtp_password").and_then(|x| x.as_str()) {
            config.smtp_password = v.to_string();
        }
        if let Some(v) = overrides.get("smtp_from").and_then(|x| x.as_str()) {
            config.smtp_from = v.to_string();
        }
        if let Some(v) = overrides.get("telegram_bot_token").and_then(|x| x.as_str()) {
            config.telegram_bot_token = v.to_string();
        }
        if let Some(v) = overrides
            .get("dingding_access_token")
            .and_then(|x| x.as_str())
        {
            config.dingding_access_token = v.to_string();
        }
        if let Some(v) = overrides.get("feishu_webhook").and_then(|x| x.as_str()) {
            config.feishu_webhook = v.to_string();
        }
        if let Some(v) = overrides.get("wecom_webhook").and_then(|x| x.as_str()) {
            config.wecom_webhook = v.to_string();
        }
        Ok(config)
    }

    /// Check if SMTP is configured
    pub fn is_smtp_configured(&self) -> bool {
        !self.smtp_host.is_empty()
            && !self.smtp_username.is_empty()
            && !self.smtp_password.is_empty()
            && !self.smtp_from.is_empty()
    }

    /// Check if Telegram is configured
    pub fn is_telegram_configured(&self) -> bool {
        !self.telegram_bot_token.is_empty()
    }

    /// Check if DingTalk is configured
    pub fn is_dingtalk_configured(&self) -> bool {
        !self.dingding_access_token.is_empty()
    }

    /// Check if Feishu is configured
    pub fn is_feishu_configured(&self) -> bool {
        !self.feishu_webhook.is_empty()
    }

    /// Check if WeCom is configured
    pub fn is_wecom_configured(&self) -> bool {
        !self.wecom_webhook.is_empty()
    }
}

/// Global static configuration instance
pub static GLOBAL_CONFIG: Lazy<RwLock<HippoxConfig>> =
    Lazy::new(|| RwLock::new(HippoxConfig::default()));

/// init global configuration from environment variables
pub fn init_config_from_env() {
    let config = HippoxConfig::load_from_env();
    let mut global = GLOBAL_CONFIG.write().unwrap();
    *global = config;
}

/// init global configuration from TOML file
pub fn init_config_from_toml_file(path: &str) -> anyhow::Result<()> {
    let config = HippoxConfig::load_from_toml_file(path)?;
    let mut global = GLOBAL_CONFIG.write().unwrap();
    *global = config;
    Ok(())
}

/// init global configuration from JSON file
pub fn init_config_from_json_file(path: &str) -> anyhow::Result<()> {
    let config = HippoxConfig::load_from_json_file(path)?;
    let mut global = GLOBAL_CONFIG.write().unwrap();
    *global = config;
    Ok(())
}

/// init global configuration from optional parameters
#[allow(clippy::too_many_arguments)]
pub fn init_config_from_params(
    lang: Option<String>,
    provider: Option<String>,
    enable_cli: Option<bool>,
    enable_tcp: Option<bool>,
    enable_http: Option<bool>,
    enable_websocket: Option<bool>,
    enable_grpc: Option<bool>,
    smtp_host: Option<String>,
    smtp_port: Option<u16>,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    smtp_from: Option<String>,
    telegram_bot_token: Option<String>,
    dingding_access_token: Option<String>,
    feishu_webhook: Option<String>,
    wecom_webhook: Option<String>,
) {
    let config = HippoxConfig::load_from_params(
        lang,
        provider,
        enable_cli,
        enable_tcp,
        enable_http,
        enable_websocket,
        enable_grpc,
        smtp_host,
        smtp_port,
        smtp_username,
        smtp_password,
        smtp_from,
        telegram_bot_token,
        dingding_access_token,
        feishu_webhook,
        wecom_webhook,
    );
    let mut global = GLOBAL_CONFIG.write().unwrap();
    *global = config;
}

/// init global configuration from JSON string of optional parameters
pub fn init_config_from_params_json_str(json_str: &str) -> anyhow::Result<()> {
    let config = HippoxConfig::load_from_params_json_str(json_str)?;
    let mut global = GLOBAL_CONFIG.write().unwrap();
    *global = config;
    Ok(())
}

/// Get a clone of the global configuration
pub fn get_config() -> HippoxConfig {
    GLOBAL_CONFIG.read().unwrap().clone()
}
