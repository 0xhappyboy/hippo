use hippox::{ConfigInitMethod, Hippox, ModelProvider, WorkflowMode};
use tempfile::tempdir;

/// Dynamic decision making, each step determined by previous result
async fn demo_react_mode(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        temp_dir.path().to_str().unwrap(),
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let user_input = "Calculate (10 + 5) * 2 - 8 / 2 + 100, then tell me the result";
    println!("Input: {}", user_input);
    let result = hippox.handle_natural_language(user_input, None).await;
    println!("Result: {}\n", result);
}

/// Execute independent tasks in parallel
async fn demo_batch_mode(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        temp_dir.path().to_str().unwrap(),
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::Batch,
    )
    .await
    .unwrap();
    let user_input = "Calculate simultaneously: 15 * 3, 100 / 4, 89 + 11, 200 - 50";
    println!("Input: {}", user_input);
    let result = hippox.handle_natural_language(user_input, None).await;
    println!("Result: {}\n", result);
}

/// Chain calculation with results passed sequentially
async fn demo_chain_mode(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        temp_dir.path().to_str().unwrap(),
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::Chain,
    )
    .await
    .unwrap();
    let user_input =
        "Start from 5, multiply by 3, subtract 2, multiply by 4, divide by 2, then add 10";
    println!("Input: {}", user_input);
    let result = hippox.handle_natural_language(user_input, None).await;
    println!("Result: {}\n", result);
}

/// One-time planning for complex tasks
async fn demo_plan_and_execute_mode(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        temp_dir.path().to_str().unwrap(),
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::PlanAndExecute,
    )
    .await
    .unwrap();
    let user_input = "Calculate 2 to the power of 4, then subtract 10, then multiply by 3. If the result is greater than 50, subtract 20, otherwise add 30";
    println!("Input: {}", user_input);
    let result = hippox.handle_natural_language(user_input, None).await;
    println!("Result: {}\n", result);
}

async fn test_skill_md_execution(api_key: &str) {
    use serde_json::json;
    use std::collections::HashMap;
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut params = HashMap::new();
    params.insert(
        "expression".to_string(),
        json!("(10 + 5) * 2 - 8 / 2 + 100"),
    );
    params.insert("show_steps".to_string(), json!(true));
    let result = hippox.handle_skill_md("math-workflow", Some(params)).await;
    println!("Result: {}", result);
}

use serde_json::json;
use std::collections::HashMap;

/// Test mortgage calculator SKILL.md
async fn test_mortgage_calculator(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut params = HashMap::new();
    params.insert("principal".to_string(), json!(300000)); // $300,000 loan
    params.insert("annual_rate".to_string(), json!(4.5)); // 4.5% interest
    params.insert("years".to_string(), json!(30)); // 30 years
    params.insert("show_breakdown".to_string(), json!(true));
    println!("Testing mortgage calculator with:");
    println!("  Principal: $300,000");
    println!("  Annual Rate: 4.5%");
    println!("  Years: 30");
    let result = hippox
        .handle_skill_md("mortgage-calculator", Some(params))
        .await;
    println!("\nResult:\n{}", result);
}

/// Test statistics analyzer SKILL.md
async fn test_stats_analyzer(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut params = HashMap::new();
    params.insert("numbers".to_string(), json!("10, 20, 30, 40, 50, 60"));
    params.insert(
        "operations".to_string(),
        json!(["sum", "average", "min", "max", "median"]),
    );
    println!("Testing statistics analyzer with numbers: 10, 20, 30, 40, 50, 60");
    let result = hippox.handle_skill_md("stats-analyzer", Some(params)).await;
    println!("\nResult:\n{}", result);
}

/// Test unit converter SKILL.md
async fn test_unit_converter(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut params = HashMap::new();
    params.insert("value".to_string(), json!(100));
    params.insert("from_unit".to_string(), json!("km"));
    params.insert("to_unit".to_string(), json!("miles"));
    params.insert("precision".to_string(), json!(2));
    println!("Testing unit converter: 100 km to miles");
    let result = hippox.handle_skill_md("unit-converter", Some(params)).await;
    println!("\nResult:\n{}", result);
}

/// Test discount calculator SKILL.md
async fn test_discount_calculator(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut params = HashMap::new();
    params.insert("original_price".to_string(), json!(100));
    params.insert("quantity".to_string(), json!(3));
    params.insert("membership_level".to_string(), json!("gold"));
    params.insert("tax_rate".to_string(), json!(8));
    println!("Testing discount calculator:");
    println!("  Original price: $100");
    println!("  Quantity: 3");
    println!("  Membership level: gold (20% discount)");
    println!("  Tax rate: 8%");
    let result = hippox
        .handle_skill_md("discount-calculator", Some(params))
        .await;
    println!("\nResult:\n{}", result);
}

/// Batch test multiple SKILL.md files
async fn test_skill_md_batch(api_key: &str, temp_dir: &tempfile::TempDir) {
    let hippox = Hippox::with_workflow_mode(
        "./skills",
        ModelProvider::DeepSeek,
        Some(api_key.to_string()),
        None,
        ConfigInitMethod::Env,
        WorkflowMode::ReAct,
    )
    .await
    .unwrap();
    let mut tasks = Vec::new();
    let mut mortgage_params = HashMap::new();
    mortgage_params.insert("principal".to_string(), json!(200000));
    mortgage_params.insert("annual_rate".to_string(), json!(5.0));
    mortgage_params.insert("years".to_string(), json!(15));
    mortgage_params.insert("show_breakdown".to_string(), json!(true));
    tasks.push(("mortgage-calculator".to_string(), Some(mortgage_params)));
    let mut unit_params = HashMap::new();
    unit_params.insert("value".to_string(), json!(50));
    unit_params.insert("from_unit".to_string(), json!("miles"));
    unit_params.insert("to_unit".to_string(), json!("km"));
    unit_params.insert("precision".to_string(), json!(2));
    tasks.push(("unit-converter".to_string(), Some(unit_params)));
    let mut discount_params = HashMap::new();
    discount_params.insert("original_price".to_string(), json!(75));
    discount_params.insert("quantity".to_string(), json!(2));
    discount_params.insert("membership_level".to_string(), json!("platinum"));
    discount_params.insert("tax_rate".to_string(), json!(0));
    tasks.push(("discount-calculator".to_string(), Some(discount_params)));
    println!("Executing 3 SKILL.md files in parallel...");
    let results = hippox.handle_skill_md_batch(tasks).await;
    for (i, result) in results.iter().enumerate() {
        println!("\n--- Result {} ---", i + 1);
        println!("{}", result);
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let temp_dir = tempdir().unwrap();
    let deep_seek_key = std::env::var("DEEP_SEEK_KEY").unwrap_or_default();
    if deep_seek_key.is_empty() {
        println!("Skipping: DEEP_SEEK_KEY not set");
        return;
    }
    demo_react_mode(&deep_seek_key, &temp_dir).await;
    demo_batch_mode(&deep_seek_key, &temp_dir).await;
    demo_chain_mode(&deep_seek_key, &temp_dir).await;
    demo_plan_and_execute_mode(&deep_seek_key, &temp_dir).await;
    test_skill_md_execution(&deep_seek_key).await;
    test_mortgage_calculator(&deep_seek_key, &temp_dir).await;
    test_stats_analyzer(&deep_seek_key, &temp_dir).await;
    test_unit_converter(&deep_seek_key, &temp_dir).await;
    test_discount_calculator(&deep_seek_key, &temp_dir).await;
    test_skill_md_batch(&deep_seek_key, &temp_dir).await;
}
