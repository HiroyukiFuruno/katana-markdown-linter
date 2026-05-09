use super::jsonc::parse_config_text;
use super::types::MarkdownLintConfig;
use crate::Error;
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

impl MarkdownLintConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(path)
            .map_err(|err| Error::new(format!("failed to read config: {err}")))?;
        let parsed = parse_config_text(&raw)?;
        Ok(Self { raw: parsed })
    }

    pub fn create_default_file(path: &Path) -> Result<Self, Error> {
        if path.exists() {
            return Self::load(path);
        }

        let config = Self::default();
        config.save(path)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let json_str = serde_json::to_string_pretty(&self.raw)
            .map_err(|err| Error::new(format!("failed to serialize config: {err}")))?;
        fs::write(path, json_str)
            .map_err(|err| Error::new(format!("failed to write config: {err}")))
    }

    pub fn create_default() -> Self {
        Self::default()
    }

    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) {
        if let Some(obj) = self.raw.as_object_mut() {
            obj.insert(rule_id.to_string(), Value::Bool(enabled));
        }
    }

    pub fn set_rule_property(&mut self, rule_id: &str, prop_key: &str, value: Value) {
        if let Some(obj) = self.raw.as_object_mut() {
            ensure_rule_object(obj, rule_id);
            if let Some(rule_obj) = obj.get_mut(rule_id).and_then(|value| value.as_object_mut()) {
                rule_obj.insert(prop_key.to_string(), value);
            }
        }
    }

    pub fn get_rule_property(&self, rule_id: &str, prop_key: &str) -> Option<&Value> {
        self.raw
            .get(rule_id)
            .and_then(|value| value.as_object())
            .and_then(|obj| obj.get(prop_key))
    }
}

fn ensure_rule_object(obj: &mut Map<String, Value>, rule_id: &str) {
    if !obj.contains_key(rule_id) || !obj[rule_id].is_object() {
        obj.insert(rule_id.to_string(), Value::Object(Map::new()));
    }
}
