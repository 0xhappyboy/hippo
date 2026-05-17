use crate::executors::Skill;
use crate::executors::types::SkillMetadata;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

static SKILL_REGISTRY: Lazy<RwLock<HashMap<String, Arc<dyn Skill>>>> = Lazy::new(|| {
    let mut registry: HashMap<String, Arc<dyn Skill>> = HashMap::new();
    registry.insert(
        "helloworld".to_string(),
        Arc::new(super::skills::HelloWorldSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "file-read".to_string(),
        Arc::new(super::skills::file::ReadFileSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "file-write".to_string(),
        Arc::new(super::skills::file::WriteFileSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "file-delete".to_string(),
        Arc::new(super::skills::file::DeleteFileSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "file-list".to_string(),
        Arc::new(super::skills::file::ListDirectorySkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "file-copy".to_string(),
        Arc::new(super::skills::file::CopyFileSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "math-calculator".to_string(),
        Arc::new(super::skills::CalculatorSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "math-power".to_string(),
        Arc::new(super::skills::PowerSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "math-statistics".to_string(),
        Arc::new(super::skills::StatisticsSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "math-unit-converter".to_string(),
        Arc::new(super::skills::UnitConverterSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "time-datetime".to_string(),
        Arc::new(super::skills::DateTimeSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "net-httprequest".to_string(),
        Arc::new(super::skills::HttpRequestSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "system-systeminfo".to_string(),
        Arc::new(super::skills::SystemInfoSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "exec-command".to_string(),
        Arc::new(super::skills::ExecCommandSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "read-url".to_string(),
        Arc::new(super::skills::ReadUrlSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "markdown-read".to_string(),
        Arc::new(super::skills::document::MarkdownReadSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "markdown-write".to_string(),
        Arc::new(super::skills::document::MarkdownWriteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "csv-read".to_string(),
        Arc::new(super::skills::document::CsvReadSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "csv-write".to_string(),
        Arc::new(super::skills::document::CsvWriteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "xml-parse".to_string(),
        Arc::new(super::skills::document::XmlParseSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "xml-to-json".to_string(),
        Arc::new(super::skills::document::XmlToJsonSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "excel-read".to_string(),
        Arc::new(super::skills::document::ExcelReadSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "excel-write".to_string(),
        Arc::new(super::skills::document::ExcelWriteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "send_email".to_string(),
        Arc::new(super::skills::message::SendEmailSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "send-telegram".to_string(),
        Arc::new(super::skills::message::SendTelegramSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "send-dingding".to_string(),
        Arc::new(super::skills::message::SendDingDingSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "send-feishu".to_string(),
        Arc::new(super::skills::message::SendFeishuSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "send-wecom".to_string(),
        Arc::new(super::skills::message::SendWecomSkill) as Arc<dyn Skill>,
    );
    // ==================== FTP Skills ====================
    registry.insert(
        "ftp-upload".to_string(),
        Arc::new(super::skills::ftp::FtpUploadSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "ftp-download".to_string(),
        Arc::new(super::skills::ftp::FtpDownloadSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "ftp-list".to_string(),
        Arc::new(super::skills::ftp::FtpListSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "ftp-delete".to_string(),
        Arc::new(super::skills::ftp::FtpDeleteSkill) as Arc<dyn Skill>,
    );
    // ==================== TCP Skills ====================
    registry.insert(
        "tcp-send".to_string(),
        Arc::new(super::skills::tcp::TcpSendSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "tcp-receive".to_string(),
        Arc::new(super::skills::tcp::TcpReceiveSkill) as Arc<dyn Skill>,
    );
    // ==================== UDP Skills ====================
    registry.insert(
        "udp-send".to_string(),
        Arc::new(super::skills::udp::UdpSendSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "udp-receive".to_string(),
        Arc::new(super::skills::udp::UdpReceiveSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "udp-broadcast".to_string(),
        Arc::new(super::skills::udp::UdpBroadcastSkill) as Arc<dyn Skill>,
    );
    // ==================== PostgreSQL Skills ====================
    registry.insert(
        "postgres-query".to_string(),
        Arc::new(super::skills::postgresql::PostgresQuerySkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "postgres-execute".to_string(),
        Arc::new(super::skills::postgresql::PostgresExecuteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "postgres-list-tables".to_string(),
        Arc::new(super::skills::postgresql::PostgresListTablesSkill) as Arc<dyn Skill>,
    );
    // ==================== MySQL Skills ====================
    registry.insert(
        "mysql-query".to_string(),
        Arc::new(super::skills::mysql::MysqlQuerySkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "mysql-execute".to_string(),
        Arc::new(super::skills::mysql::MysqlExecuteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "mysql-list-tables".to_string(),
        Arc::new(super::skills::mysql::MysqlListTablesSkill) as Arc<dyn Skill>,
    );
    // ==================== Redis Skills ====================
    registry.insert(
        "redis-set".to_string(),
        Arc::new(super::skills::redis::RedisSetSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "redis-get".to_string(),
        Arc::new(super::skills::redis::RedisGetSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "redis-del".to_string(),
        Arc::new(super::skills::redis::RedisDelSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "redis-keys".to_string(),
        Arc::new(super::skills::redis::RedisKeysSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "redis-hset".to_string(),
        Arc::new(super::skills::redis::RedisHSetSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "redis-hget".to_string(),
        Arc::new(super::skills::redis::RedisHGetSkill) as Arc<dyn Skill>,
    );
    // ==================== SQLite Skills ====================
    registry.insert(
        "sqlite-query".to_string(),
        Arc::new(super::skills::sqlite::SqliteQuerySkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "sqlite-execute".to_string(),
        Arc::new(super::skills::sqlite::SqliteExecuteSkill) as Arc<dyn Skill>,
    );
    registry.insert(
        "sqlite-list-tables".to_string(),
        Arc::new(super::skills::sqlite::SqliteListTablesSkill) as Arc<dyn Skill>,
    );
    RwLock::new(registry)
});

/// Generate AI registry JSON for LLM
pub fn generate_ai_registry() -> Vec<SkillMetadata> {
    let registry = get_registry();
    registry
        .values()
        .map(|skill| skill.get_metadata())
        .collect()
}

/// generate skill registry table json
pub fn generate_skill_registry_table_json() -> Value {
    let metadata = generate_ai_registry();
    serde_json::json!({
        "version": "1.0",
        "total_skills": metadata.len(),
        "skills": metadata,
        "instruction": r#"You can call a skill by returning a JSON object with 'action' and 'parameters' fields. Example: {"action": "calculator", "parameters": {"expression": "2+3"}}"#
    })
}

/// generate skill registry table json string
pub fn generate_skill_registry_table_json_str() -> String {
    serde_json::to_string_pretty(&generate_skill_registry_table_json()).unwrap()
}

pub fn get_registry() -> std::sync::RwLockReadGuard<'static, HashMap<String, Arc<dyn Skill>>> {
    SKILL_REGISTRY.read().unwrap()
}

pub fn get_registry_mut() -> std::sync::RwLockWriteGuard<'static, HashMap<String, Arc<dyn Skill>>> {
    SKILL_REGISTRY.write().unwrap()
}

pub fn get_skill(name: &str) -> Option<Arc<dyn Skill>> {
    get_registry().get(name).cloned()
}

pub fn register_skill(name: String, skill: Arc<dyn Skill>) {
    get_registry_mut().insert(name, skill);
}

pub fn has_skill(name: &str) -> bool {
    get_registry().contains_key(name)
}

pub fn list_skills() -> Vec<String> {
    get_registry().keys().cloned().collect()
}

#[cfg(test)]
mod registry_test {
    use super::*;

    #[test]
    fn test_generate_ai_registry() {
        let metadata = generate_ai_registry();
        println!("Total skills: {}", metadata.len());
        for skill in &metadata {
            println!(
                "  - {} ({}): {}",
                skill.name, skill.category, skill.description
            );
        }
    }

    #[test]
    fn test_print_all_skill_json() {
        let json_str = generate_skill_registry_table_json();
        println!("{:?}", json_str);
    }

    #[test]
    fn test_print_all_skill_json_str() {
        let json_str = generate_skill_registry_table_json_str();
        println!("{:?}", json_str);
    }
}
