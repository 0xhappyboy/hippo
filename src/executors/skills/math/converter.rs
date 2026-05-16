use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::executors::{skills::common, types::Skill};

#[derive(Debug)]
pub struct UnitConverterSkill;

#[async_trait::async_trait]
impl Skill for UnitConverterSkill {
    fn name(&self) -> &str {
        "unit_converter"
    }

    fn description(&self) -> &str {
        "Convert between units. Parameters: value (required), from (required) - source unit, to (required) - target unit"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        let value_str = parameters
            .get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'value' parameter"))?;
        let from_unit = parameters
            .get("from")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'from' parameter"))?
            .to_lowercase();
        let to_unit = parameters
            .get("to")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'to' parameter"))?
            .to_lowercase();
        let value = common::Math::validate_number(value_str)?;
        let result = convert_units(value, &from_unit, &to_unit)?;
        let precision = parameters
            .get("precision")
            .and_then(|v| v.as_u64())
            .unwrap_or(2);
        Ok(format!(
            "{} {} = {} {}",
            value,
            from_unit,
            common::Math::format_number(result, precision as usize),
            to_unit
        ))
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        parameters
            .get("value")
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: value"))?;
        parameters
            .get("from")
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: from"))?;
        parameters
            .get("to")
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: to"))?;
        Ok(())
    }
}

fn convert_units(value: f64, from: &str, to: &str) -> Result<f64> {
    let to_meters = |unit: &str, val: f64| -> Result<f64> {
        match unit {
            "m" | "meter" | "meters" => Ok(val),
            "km" | "kilometer" | "kilometers" => Ok(val * 1000.0),
            "cm" | "centimeter" | "centimeters" => Ok(val / 100.0),
            "mm" | "millimeter" | "millimeters" => Ok(val / 1000.0),
            "mi" | "mile" | "miles" => Ok(val * 1609.344),
            "ft" | "foot" | "feet" => Ok(val * 0.3048),
            "in" | "inch" | "inches" => Ok(val * 0.0254),
            _ => anyhow::bail!("Unknown length unit: {}", unit),
        }
    };
    let from_meters = to_meters(from, value)?;
    let from_meters_to_target = |unit: &str, val: f64| -> Result<f64> {
        match unit {
            "m" | "meter" | "meters" => Ok(val),
            "km" | "kilometer" | "kilometers" => Ok(val / 1000.0),
            "cm" | "centimeter" | "centimeters" => Ok(val * 100.0),
            "mm" | "millimeter" | "millimeters" => Ok(val * 1000.0),
            "mi" | "mile" | "miles" => Ok(val / 1609.344),
            "ft" | "foot" | "feet" => Ok(val / 0.3048),
            "in" | "inch" | "inches" => Ok(val / 0.0254),
            _ => anyhow::bail!("Unknown length unit: {}", unit),
        }
    };
    from_meters_to_target(to, from_meters)
}
