use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::executors::{skills::common, types::Skill};

#[derive(Debug)]
pub struct StatisticsSkill;

#[async_trait::async_trait]
impl Skill for StatisticsSkill {
    fn name(&self) -> &str {
        "math_statistics"
    }

    fn description(&self) -> &str {
        "Calculate statistical values. Parameters: numbers (required) - array of numbers, operation (required) - mean/median/mode/sum/min/max"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let numbers_json = parameters
            .get("numbers")
            .ok_or_else(|| anyhow::anyhow!("Missing 'numbers' parameter"))?;
        let numbers_array = numbers_json
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("'numbers' must be an array"))?;
        let mut numbers = Vec::new();
        for num in numbers_array {
            let value = num
                .as_f64()
                .or_else(|| num.as_str().and_then(|s| s.parse::<f64>().ok()))
                .ok_or_else(|| anyhow::anyhow!("Invalid number in array: {:?}", num))?;
            numbers.push(value);
        }
        if numbers.is_empty() {
            anyhow::bail!("Numbers array is empty");
        }
        let operation = parameters
            .get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'operation' parameter"))?;
        let result = match operation {
            "sum" => numbers.iter().sum::<f64>(),
            "mean" | "average" => numbers.iter().sum::<f64>() / numbers.len() as f64,
            "min" => numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            "max" => numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            "median" => {
                let mut sorted = numbers.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mid = sorted.len() / 2;
                if sorted.len() % 2 == 0 {
                    (sorted[mid - 1] + sorted[mid]) / 2.0
                } else {
                    sorted[mid]
                }
            }
            "mode" => {
                use std::collections::HashMap;
                let mut counts = HashMap::new();
                for &num in &numbers {
                    *counts.entry(num.to_string()).or_insert(0) += 1;
                }
                let max_count = *counts.values().max().unwrap_or(&0);
                let modes: Vec<_> = counts
                    .iter()
                    .filter(|(_, count)| **count == max_count)
                    .map(|(num, _)| num.clone())
                    .collect();
                return Ok(format!("Mode: {}", modes.join(", ")));
            }
            _ => anyhow::bail!("Unknown operation: {}", operation),
        };
        let precision = parameters
            .get("precision")
            .and_then(|v| v.as_u64())
            .unwrap_or(2);
        Ok(format!(
            "{} = {}",
            operation,
            common::Math::format_number(result, precision as usize)
        ))
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("numbers")
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: numbers"))?;
        parameters
            .get("operation")
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: operation"))?;
        Ok(())
    }
}
