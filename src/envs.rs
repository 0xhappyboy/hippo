/// Language setting (en/zh)
pub const HIPPOX_LANG: &str = "HIPPOX_LANG";

/// LLM provider (openai, deepseek, anthropic, google)
pub const HIPPOX_PROVIDER: &str = "HIPPOX_PROVIDER";

/// Enable CLI interface (true/false)
pub const HIPPOX_ENABLE_CLI: &str = "HIPPOX_ENABLE_CLI";

/// Enable TCP server (true/false)
pub const HIPPOX_ENABLE_TCP: &str = "HIPPOX_ENABLE_TCP";

/// Enable HTTP server (true/false)
pub const HIPPOX_ENABLE_HTTP: &str = "HIPPOX_ENABLE_HTTP";

/// Enable WebSocket server (true/false)
pub const HIPPOX_ENABLE_WS: &str = "HIPPOX_ENABLE_WS";

/// SMTP server hostname (e.g., smtp.gmail.com)
pub const HIPPOX_SMTP_HOST: &str = "HIPPOX_SMTP_HOST";

/// SMTP server port (e.g., 587 for TLS, 465 for SSL)
pub const HIPPOX_SMTP_PORT: &str = "HIPPOX_SMTP_PORT";

/// SMTP authentication username (usually email address)
pub const HIPPOX_SMTP_USERNAME: &str = "HIPPOX_SMTP_USERNAME";

/// SMTP authentication password or app-specific password
pub const HIPPOX_SMTP_PASSWORD: &str = "HIPPOX_SMTP_PASSWORD";

/// Default sender email address
pub const HIPPOX_SMTP_FROM: &str = "HIPPOX_SMTP_FROM";

/// Telegram bot token (format: 1234567890:ABCdefGHIJKLMNopqrsTUVwxyz)
pub const HIPPOX_TELEGRAM_BOT_TOKEN: &str = "HIPPOX_TELEGRAM_BOT_TOKEN";

/// DingDing robot access token
pub const HIPPOX_DINGDING_ACCESS_TOKEN: &str = "HIPPOX_DINGDING_ACCESS_TOKEN";

/// Feishu bot webhook URL
pub const HIPPOX_FEISHU_WEBHOOK: &str = "HIPPOX_FEISHU_WEBHOOK";

/// WeCom (Enterprise WeChat) robot webhook URL
pub const HIPPOX_WECOM_WEBHOOK: &str = "HIPPOX_WECOM_WEBHOOK";

/// Get environment variable value with optional default
pub fn get_env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

/// Get environment variable or return default
pub fn get_env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Check if environment variable is set to "true"
pub fn is_env_true(key: &str) -> bool {
    std::env::var(key).unwrap_or_else(|_| "false".to_string()) == "true"
}

/// Get required environment variable, returns error if not set
pub fn get_required_env(key: &str) -> anyhow::Result<String> {
    std::env::var(key).map_err(|_| anyhow::anyhow!("Environment variable '{}' is not set", key))
}
