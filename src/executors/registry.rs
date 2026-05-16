use crate::executors::Skill;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

static SKILL_REGISTRY: Lazy<RwLock<HashMap<String, Arc<dyn Skill>>>> = Lazy::new(|| {
    let mut registry: HashMap<String, Arc<dyn Skill>> = HashMap::new();
    registry.insert(
        "helloworld".to_string(),
        Arc::new(super::skills::HelloWorldSkill) as Arc<dyn Skill>,
    );
    RwLock::new(registry)
});

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
