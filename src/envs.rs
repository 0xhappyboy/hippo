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

// ==================== FTP Configuration ====================
/// FTP server hostname
pub const HIPPOX_FTP_HOST: &str = "HIPPOX_FTP_HOST";
/// FTP server port
pub const HIPPOX_FTP_PORT: &str = "HIPPOX_FTP_PORT";
/// FTP username
pub const HIPPOX_FTP_USERNAME: &str = "HIPPOX_FTP_USERNAME";
/// FTP password
pub const HIPPOX_FTP_PASSWORD: &str = "HIPPOX_FTP_PASSWORD";
/// FTP default remote directory
pub const HIPPOX_FTP_REMOTE_DIR: &str = "HIPPOX_FTP_REMOTE_DIR";
/// FTP connection timeout (seconds)
pub const HIPPOX_FTP_TIMEOUT: &str = "HIPPOX_FTP_TIMEOUT";
/// FTP transfer mode (binary/ascii)
pub const HIPPOX_FTP_MODE: &str = "HIPPOX_FTP_MODE";

// ==================== TCP Configuration ====================
/// TCP default host
pub const HIPPOX_TCP_HOST: &str = "HIPPOX_TCP_HOST";
/// TCP default port
pub const HIPPOX_TCP_PORT: &str = "HIPPOX_TCP_PORT";
/// TCP default timeout (seconds)
pub const HIPPOX_TCP_TIMEOUT: &str = "HIPPOX_TCP_TIMEOUT";
/// TCP default encoding (utf8, hex, base64)
pub const HIPPOX_TCP_ENCODING: &str = "HIPPOX_TCP_ENCODING";

// ==================== UDP Configuration ====================
/// UDP default host
pub const HIPPOX_UDP_HOST: &str = "HIPPOX_UDP_HOST";
/// UDP default port
pub const HIPPOX_UDP_PORT: &str = "HIPPOX_UDP_PORT";
/// UDP default timeout (seconds)
pub const HIPPOX_UDP_TIMEOUT: &str = "HIPPOX_UDP_TIMEOUT";
/// UDP default encoding (utf8, hex, base64)
pub const HIPPOX_UDP_ENCODING: &str = "HIPPOX_UDP_ENCODING";
/// UDP enable broadcast
pub const HIPPOX_UDP_BROADCAST: &str = "HIPPOX_UDP_BROADCAST";

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
