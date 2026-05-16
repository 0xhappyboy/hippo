use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::executors::{skills::common, types::Skill};

#[derive(Debug)]
pub struct PowerSkill;

#[async_trait::async_trait]
impl Skill for PowerSkill {
    fn name(&self) -> &str {
        "math_power"
    }

    fn description(&self) -> &str {
        "Calculate power or root. Parameters: base (required), exponent (required) for power, or use sqrt for square root"
    }

    async fn execute(&self, parameters: &HashMap<String, Value>) -> Result<String> {
        if let Some(value) = parameters.get("sqrt").and_then(|v| v.as_str()) {
            let num = common::Math::validate_number(value)?;
            if num < 0.0 {
                anyhow::bail!("Cannot calculate square root of negative number: {}", num);
            }
            let result = num.sqrt();
            return Ok(format!("√{} = {}", num, result));
        }
        if let Some(value) = parameters.get("cbrt").and_then(|v| v.as_str()) {
            let num = common::Math::validate_number(value)?;
            let result = num.cbrt();
            return Ok(format!("∛{} = {}", num, result));
        }
        let base = parameters
            .get("base")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'base' parameter"))?;
        let exponent = parameters
            .get("exponent")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'exponent' parameter"))?;
        let base_num = common::Math::validate_number(base)?;
        let exp_num = common::Math::validate_number(exponent)?;
        let result = base_num.powf(exp_num);
        let precision = parameters
            .get("precision")
            .and_then(|v| v.as_u64())
            .unwrap_or(2);
        Ok(format!(
            "{} ^ {} = {}",
            base_num,
            exp_num,
            common::Math::format_number(result, precision as usize)
        ))
    }

    fn validate(&self, parameters: &HashMap<String, Value>) -> Result<()> {
        // Either (base + exponent) OR sqrt OR cbrt
        let has_power = parameters.contains_key("base") && parameters.contains_key("exponent");
        let has_sqrt = parameters.contains_key("sqrt");
        let has_cbrt = parameters.contains_key("cbrt");

        if !has_power && !has_sqrt && !has_cbrt {
            anyhow::bail!("Missing parameters: provide (base + exponent) or (sqrt) or (cbrt)");
        }
        Ok(())
    }
}
