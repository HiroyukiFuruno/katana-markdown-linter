use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[rustfmt::skip]
pub(crate) const EXPECTED_CODES: &[&str] = &["en", "ja", "zh-CN", "zh-TW", "ko", "pt", "fr", "de", "es", "it"];

#[rustfmt::skip]
const REQUIRED_DOC_LABELS: &[&str] = &[
    "tags", "aliases", "parameters", "none", "setting", "default", "overview", "config",
    "examples", "rationale", "overview_text", "config_text", "rationale_text", "period",
];
#[derive(Debug, Deserialize)]
pub(crate) struct LanguageEntry {
    pub(crate) code: String,
    name: String,
}
#[derive(Debug, Deserialize)]
pub(crate) struct LocaleCatalog {
    pub(crate) rule_descriptions: BTreeMap<String, String>,
    pub(crate) rule_doc: RuleDocCatalog,
}
#[derive(Debug, Deserialize)]
pub(crate) struct RuleDocCatalog {
    pub(crate) generation: String,
    pub(crate) labels: BTreeMap<String, String>,
}
pub(crate) fn locale_dir() -> PathBuf {
    super::super::workspace_root().join("src/i18n/locales")
}

pub(crate) fn read_languages(
    locale_dir: &Path,
    violations: &mut Vec<String>,
) -> Vec<LanguageEntry> {
    read_json(&locale_dir.join("languages.json"), violations).unwrap_or_default()
}

pub(crate) fn read_catalog(
    locale_dir: &Path,
    code: &str,
    violations: &mut Vec<String>,
) -> Option<LocaleCatalog> {
    read_json(&locale_dir.join(format!("{code}.json")), violations)
}

pub(crate) fn validate_languages(
    languages: &[LanguageEntry],
    violations: &mut Vec<String>,
) -> BTreeSet<String> {
    let mut codes = BTreeSet::new();
    for (index, language) in languages.iter().enumerate() {
        if language.code.is_empty() || language.name.is_empty() {
            violations.push(format!(
                "src/i18n/locales/languages.json: entry {index} must have code and name"
            ));
        }
        if !codes.insert(language.code.clone()) {
            violations.push(format!(
                "src/i18n/locales/languages.json: duplicate code `{}`",
                language.code
            ));
        }
    }
    compare_sets(
        "languages.json KatanA locale set",
        &EXPECTED_CODES
            .iter()
            .map(|code| (*code).to_string())
            .collect(),
        &codes,
        violations,
    );
    codes
}

pub(crate) fn locale_file_codes(
    locale_dir: &Path,
    violations: &mut Vec<String>,
) -> BTreeSet<String> {
    let mut codes = BTreeSet::new();
    match std::fs::read_dir(locale_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                collect_locale_file_code(&entry.path(), &mut codes);
            }
        }
        Err(err) => violations.push(format!("{}: {err}", locale_dir.display())),
    }
    codes
}

fn collect_locale_file_code(path: &Path, codes: &mut BTreeSet<String>) {
    if path.file_name().and_then(|name| name.to_str()) == Some("languages.json") {
        return;
    }
    if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
        if let Some(code) = path.file_stem().and_then(|stem| stem.to_str()) {
            codes.insert(code.to_string());
        }
    }
}

pub(crate) fn compare_sets(
    label: &str,
    expected: &BTreeSet<String>,
    actual: &BTreeSet<String>,
    violations: &mut Vec<String>,
) {
    for missing in expected.difference(actual) {
        violations.push(format!("i18n: {label} missing `{missing}`"));
    }
    for extra in actual.difference(expected) {
        violations.push(format!("i18n: {label} has unexpected `{extra}`"));
    }
}

pub(crate) fn active_rule_ids() -> BTreeSet<String> {
    katana_markdown_linter::RuleCatalogService::available_rules()
        .into_iter()
        .map(|rule| rule.id)
        .collect()
}

pub(crate) fn validate_rule_doc_catalog(
    code: &str,
    rule_doc: &RuleDocCatalog,
    violations: &mut Vec<String>,
) {
    if !matches!(
        rule_doc.generation.as_str(),
        "canonical" | "manual" | "generated"
    ) {
        violations.push(format!("{code}.json: unknown rule_doc generation"));
    }
    for key in REQUIRED_DOC_LABELS {
        if rule_doc
            .labels
            .get(*key)
            .is_none_or(|value| value.is_empty())
        {
            violations.push(format!("{code}.json: missing rule_doc.labels.{key}"));
        }
    }
}

fn read_json<T: DeserializeOwned>(path: &Path, violations: &mut Vec<String>) -> Option<T> {
    match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(value) => Some(value),
            Err(err) => {
                violations.push(format!("{}: invalid JSON: {err}", path.display()));
                None
            }
        },
        Err(err) => {
            violations.push(format!("{}: {err}", path.display()));
            None
        }
    }
}
