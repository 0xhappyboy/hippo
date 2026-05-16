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
