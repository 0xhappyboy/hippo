use anyhow::Result;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;

/// HTTP common module
///
/// This module provides a reusable HTTP client that can be used by other skills.
///
/// # Examples
///
/// ## Parse from skill parameters
///
/// ```rust
/// use crate::executors::utils::Http;
/// use std::collections::HashMap;
/// use serde_json::json;
///
/// let mut params = HashMap::new();
/// params.insert("url".to_string(), json!("https://api.example.com/data"));
/// params.insert("method".to_string(), json!("GET"));
///
/// let config = Http::parse_config(&params)?;
/// let response = Http::execute(&config).await?;
/// println!("{}", response.to_formatted_string());
/// ```
///
/// ## Build config manually
///
/// ```rust
/// use crate::executors::utils::Http;
///
/// let config = Http::RequestConfig {
///     url: "https://api.weather.com/v1/current".to_string(),
///     method: "POST".to_string(),
///     headers: Some([
///         ("Authorization".to_string(), "Bearer token".to_string()),
///     ].into()),
///     body: Some(r#"{"city": "Beijing"}"#.to_string()),
///     timeout_secs: Some(10),
/// };
///
/// let response = Http::execute(&config).await?;
/// ```
pub mod Http {
    use super::*;

    /// HTTP request configuration
    #[derive(Debug, Clone)]
    pub struct RequestConfig {
        pub url: String,
        pub method: String,
        pub headers: Option<HashMap<String, String>>,
        pub body: Option<String>,
        pub timeout_secs: Option<u64>,
    }

    impl Default for RequestConfig {
        fn default() -> Self {
            Self {
                url: String::new(),
                method: "GET".to_string(),
                headers: None,
                body: None,
                timeout_secs: Some(30),
            }
        }
    }

    /// HTTP response result
    #[derive(Debug, Clone)]
    pub struct Response {
        pub status: u16,
        pub body: String,
        pub is_success: bool,
    }

    impl Response {
        pub fn to_formatted_string(&self) -> String {
            if self.is_success {
                if let Ok(json) = serde_json::from_str::<Value>(&self.body) {
                    format!(
                        "HTTP {}:\n{}",
                        self.status,
                        serde_json::to_string_pretty(&json).unwrap_or(self.body.clone())
                    )
                } else {
                    format!("HTTP {}:\n{}", self.status, self.body)
                }
            } else {
                format!("HTTP Error {}: {}", self.status, self.body)
            }
        }
    }

    /// Execute HTTP request
    pub async fn execute(config: &RequestConfig) -> Result<Response> {
        let timeout = std::time::Duration::from_secs(config.timeout_secs.unwrap_or(30));
        let client = Client::builder().timeout(timeout).build()?;
        let method = config.method.to_uppercase();
        let mut request_builder = match method.as_str() {
            "GET" => client.get(&config.url),
            "POST" => client.post(&config.url),
            "PUT" => client.put(&config.url),
            "DELETE" => client.delete(&config.url),
            "PATCH" => client.patch(&config.url),
            _ => anyhow::bail!("Unsupported HTTP method: {}", method),
        };
        if let Some(headers) = &config.headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                if let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) {
                    if let Ok(header_value) = HeaderValue::from_str(value) {
                        header_map.insert(header_name, header_value);
                    }
                }
            }
            request_builder = request_builder.headers(header_map);
        }
        if let Some(body) = &config.body {
            request_builder = request_builder.body(body.clone());
        }
        let response = request_builder.send().await?;
        let status = response.status().as_u16();
        let body = response.text().await?;
        let is_success = status >= 200 && status < 300;
        Ok(Response {
            status,
            body,
            is_success,
        })
    }

    /// Parse parameters from Skill parameters into RequestConfig
    pub fn parse_config(parameters: &HashMap<String, Value>) -> Result<RequestConfig> {
        let url = parameters
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'url' parameter"))?
            .to_string();
        let method = parameters
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET")
            .to_string();
        let timeout_secs = parameters.get("timeout").and_then(|v| v.as_u64());
        let headers = parameters
            .get("headers")
            .and_then(|v| v.as_object())
            .map(|obj| {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    if let Some(val_str) = v.as_str() {
                        map.insert(k.clone(), val_str.to_string());
                    }
                }
                map
            });
        let body = parameters.get("body").map(|v| {
            if v.is_string() {
                v.as_str().unwrap_or("").to_string()
            } else {
                v.to_string()
            }
        });
        Ok(RequestConfig {
            url,
            method,
            headers,
            body,
            timeout_secs,
        })
    }
}

/// File common module
///
/// This module provides reusable file system utilities that can be used by file-related skills.
///
/// # Examples
///
/// ## Validate and sanitize file path
///
/// ```rust
/// use crate::executors::utils::File;
///
/// let safe_path = File::validate_path("documents/readme.txt", None)?;
/// ```
///
/// ## Check if file exists
///
/// ```rust
/// use crate::executors::utils::File;
///
/// if File::file_exists("/tmp/test.txt") {
///     let content = File::read_file_content("/tmp/test.txt")?;
///     println!("{}", content);
/// }
/// ```
///
/// ## Read and write files
///
/// ```rust
/// use crate::executors::utils::File;
///
/// // Ensure directory exists
/// File::ensure_dir("/tmp/myapp/logs")?;
///
/// // Write file
/// File::write_file_content("/tmp/myapp/logs/app.log", "Hello World", false)?;
///
/// // Read file
/// let content = File::read_file_content("/tmp/myapp/logs/app.log")?;
/// ```
///
/// ## Get file metadata
///
/// ```rust
/// use crate::executors::utils::File;
///
/// let metadata = File::get_file_metadata("/tmp/test.txt")?;
/// println!("File size: {} bytes", metadata.len());
/// ```
pub mod File {
    use anyhow::Result;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Validate and sanitize file path to prevent directory traversal attacks
    pub fn validate_path(path: &str, base_dir: Option<&str>) -> Result<PathBuf> {
        let path_buf = PathBuf::from(path);
        if path_buf
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            anyhow::bail!("Path traversal not allowed: {}", path);
        }
        if let Some(base) = base_dir {
            let full_path = Path::new(base).join(&path_buf);
            let canonicalized = fs::canonicalize(&full_path)?;
            let base_canonical = fs::canonicalize(base)?;

            if !canonicalized.starts_with(base_canonical) {
                anyhow::bail!("Path is outside base directory: {}", path);
            }
            Ok(canonicalized)
        } else {
            Ok(path_buf)
        }
    }

    /// Check if a file exists
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists() && Path::new(path).is_file()
    }

    /// Check if a directory exists
    pub fn dir_exists(path: &str) -> bool {
        Path::new(path).exists() && Path::new(path).is_dir()
    }

    /// Ensure directory exists, create if not
    pub fn ensure_dir(path: &str) -> Result<()> {
        let dir = Path::new(path);
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }

    /// Get file extension
    pub fn get_extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
    }

    /// Read file content as string
    pub fn read_file_content(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    /// Write content to file
    pub fn write_file_content(path: &str, content: &str, append: bool) -> Result<()> {
        if append {
            fs::write(path, content)?;
        } else {
            fs::write(path, content)?;
        }
        Ok(())
    }

    /// Get file metadata
    pub fn get_file_metadata(path: &str) -> Result<fs::Metadata> {
        let metadata = fs::metadata(path)?;
        Ok(metadata)
    }
}

/// Math common module
///
/// This module provides reusable mathematical utilities that can be used by math-related skills.
///
/// # Examples
///
/// ## Validate and parse numbers
///
/// ```rust
/// use crate::executors::utils::Math;
///
/// let num = Math::validate_number("3.14")?;
/// let integer = Math::validate_integer("42")?;
/// ```
///
/// ## Format numbers with precision
///
/// ```rust
/// use crate::executors::utils::Math;
///
/// let formatted = Math::format_number(3.1415926, 2);
/// assert_eq!(formatted, "3.14");
/// ```
///
/// ## Check if number is within range
///
/// ```rust
/// use crate::executors::utils::Math;
///
/// let is_in_range = Math::in_range(5.0, 0.0, 10.0);
/// assert!(is_in_range);
/// ```
///
/// ## Complete example in a skill
///
/// ```rust
/// use crate::executors::types::Skill;
/// use crate::executors::utils::Math;
///
/// async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
///     let value = parameters
///         .get("value")
///         .and_then(|v| v.as_str())
///         .ok_or_else(|| anyhow::anyhow!("Missing 'value' parameter"))?;
///     let num = Math::validate_number(value)?;
///     let result = num * 2.0;
///     let precision = parameters
///         .get("precision")
///         .and_then(|v| v.as_u64())
///         .unwrap_or(2);
///     Ok(Math::format_number(result, precision as usize))
/// }
/// ```
pub mod Math {
    use anyhow::Result;

    /// Validate numeric input
    pub fn validate_number(value: &str) -> Result<f64> {
        value
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid number: {}", value))
    }

    /// Validate integer input
    pub fn validate_integer(value: &str) -> Result<i64> {
        value
            .parse::<i64>()
            .map_err(|_| anyhow::anyhow!("Invalid integer: {}", value))
    }

    /// Format number with appropriate precision
    pub fn format_number(value: f64, precision: usize) -> String {
        format!("{:.1$}", value, precision)
    }

    /// Check if number is within range
    pub fn in_range(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }
}
