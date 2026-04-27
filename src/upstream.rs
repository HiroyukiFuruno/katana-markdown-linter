use crate::rules::markdown::RulePropertyType;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleDocument {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub properties: Vec<UpstreamRuleProperty>,
    pub examples: Vec<UpstreamRuleExample>,
    pub fixable: Option<bool>,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleExample {
    pub language: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleProperty {
    pub key: String,
    pub value_type: String,
    pub default_value: Option<String>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamCatalog {
    pub source: String,
    pub rules: Vec<UpstreamRuleDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrix {
    pub upstream_source: String,
    pub summary: RuleFixtureMatrixSummary,
    pub rules: Vec<RuleFixtureEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrixSummary {
    pub total_rules: usize,
    pub rules_with_examples: usize,
    pub rules_with_fix_metadata: usize,
    pub rules_with_parameters: usize,
    pub manual_required: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureEntry {
    pub rule_id: String,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<UpstreamRuleProperty>,
    pub fixable: Option<bool>,
    pub check_pass: Vec<RuleFixtureCase>,
    pub check_fail: Vec<RuleFixtureCase>,
    pub fix: Vec<RuleFixtureCase>,
    pub config_valid: Vec<RuleFixtureCase>,
    pub config_invalid: Vec<RuleFixtureCase>,
    pub edge: Vec<RuleFixtureCase>,
    pub manual_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureCase {
    pub name: String,
    pub source: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftReport {
    pub upstream_source: String,
    pub summary: RuleFixtureDriftSummary,
    pub items: Vec<RuleFixtureDriftItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftSummary {
    pub upstream_rules: usize,
    pub matrix_rules: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
    pub manual_required: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftItem {
    pub rule_id: String,
    pub drift_type: RuleFixtureDriftType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleFixtureDriftType {
    MissingFixture,
    StaleFixture,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DriftReport {
    pub upstream_source: String,
    pub summary: DriftSummary,
    pub items: Vec<DriftItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct DriftSummary {
    pub upstream_rules: usize,
    pub local_rules: usize,
    pub missing: usize,
    pub removed: usize,
    pub deprecated: usize,
    pub mismatches: usize,
    pub unknown_needs_review: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DriftItem {
    pub rule_id: String,
    pub drift_type: DriftType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftType {
    MissingLocalRule,
    RemovedUpstreamRule,
    DeprecatedUpstreamRule,
    PropertyMissingLocal,
    PropertyMissingUpstream,
    PropertyTypeMismatch,
    PropertyDefaultMismatch,
    SummaryMismatch,
    UnknownNeedsReview,
}

pub fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
    let mut lines = source.lines();
    let header = lines
        .next()
        .ok_or_else(|| "rule document is empty".to_string())?;
    let (id, summary) = parse_header(header)?;
    let mut tags = Vec::new();
    let mut aliases = Vec::new();
    let mut parameter_lines = Vec::new();
    let mut in_parameters = false;

    for line in source.lines().skip(1) {
        if let Some(value) = line.strip_prefix("Tags:") {
            tags = parse_backtick_list(value);
            continue;
        }
        if let Some(value) = line.strip_prefix("Aliases:") {
            aliases = parse_backtick_list(value);
            continue;
        }
        if line.trim() == "Parameters:" {
            in_parameters = true;
            continue;
        }
        if in_parameters {
            if line.starts_with("- `") || line.starts_with("  ") {
                parameter_lines.push(line.to_string());
            } else if !line.trim().is_empty() {
                in_parameters = false;
            }
        }
    }

    let properties = parse_parameter_block(&parameter_lines);
    let examples = parse_fenced_examples(source);
    let name = aliases
        .first()
        .cloned()
        .unwrap_or_else(|| summary.to_lowercase());
    let fixable = if source.contains("Fixable: Some violations can be fixed")
        || source.contains("automatically fixable")
        || source.contains("can be fixed")
    {
        Some(true)
    } else {
        None
    };

    Ok(UpstreamRuleDocument {
        id,
        name,
        summary,
        tags,
        aliases,
        properties,
        examples,
        fixable,
        source_path: None,
    })
}

pub fn load_catalog_from_dir(doc_dir: &Path) -> Result<UpstreamCatalog, String> {
    let mut rules = Vec::new();
    for entry in fs::read_dir(doc_dir).map_err(|err| format!("{}: {err}", doc_dir.display()))? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if !is_rule_doc(&path) {
            continue;
        }
        let source =
            fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut document = parse_rule_document(&source)?;
        document.source_path = Some(path);
        rules.push(document);
    }
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(UpstreamCatalog {
        source: "DavidAnson/markdownlint default branch".to_string(),
        rules,
    })
}

pub fn build_fixture_matrix(catalog: &UpstreamCatalog) -> RuleFixtureMatrix {
    let mut rules = catalog
        .rules
        .iter()
        .map(rule_fixture_entry)
        .collect::<Vec<_>>();
    rules.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));

    let mut summary = RuleFixtureMatrixSummary {
        total_rules: rules.len(),
        ..RuleFixtureMatrixSummary::default()
    };
    for entry in &rules {
        if !entry.check_fail.is_empty() || !entry.check_pass.is_empty() {
            summary.rules_with_examples += 1;
        }
        if entry.fixable.is_some() {
            summary.rules_with_fix_metadata += 1;
        }
        if !entry.parameters.is_empty() {
            summary.rules_with_parameters += 1;
        }
        if !entry.manual_required.is_empty() {
            summary.manual_required += 1;
        }
    }

    let mut matrix = RuleFixtureMatrix {
        upstream_source: catalog.source.clone(),
        summary,
        rules,
    };
    let drift = compare_fixture_matrix(catalog, &matrix);
    matrix.summary.missing_fixtures = drift.summary.missing_fixtures;
    matrix.summary.stale_fixtures = drift.summary.stale_fixtures;
    matrix
}

pub fn render_fixture_matrix_summary(matrix: &RuleFixtureMatrix) -> String {
    let mut out = String::new();
    out.push_str("# Rule Fixture Matrix\n\n");
    out.push_str(&format!("- total rules: {}\n", matrix.summary.total_rules));
    out.push_str(&format!(
        "- rules with examples: {}\n",
        matrix.summary.rules_with_examples
    ));
    out.push_str(&format!(
        "- rules with fix metadata: {}\n",
        matrix.summary.rules_with_fix_metadata
    ));
    out.push_str(&format!(
        "- rules with parameters: {}\n",
        matrix.summary.rules_with_parameters
    ));
    out.push_str(&format!(
        "- manual required: {}\n\n",
        matrix.summary.manual_required
    ));
    out.push_str(&format!(
        "- missing fixtures: {}\n",
        matrix.summary.missing_fixtures
    ));
    out.push_str(&format!(
        "- stale fixtures: {}\n\n",
        matrix.summary.stale_fixtures
    ));
    out.push_str("| Rule | Aliases | Parameters | Examples | Fixable | Manual Required |\n");
    out.push_str("| --- | --- | ---: | ---: | --- | --- |\n");
    for rule in &matrix.rules {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            rule.rule_id,
            rule.aliases.join(", ").replace('|', "\\|"),
            rule.parameters.len(),
            rule.check_fail.len() + rule.check_pass.len(),
            rule.fixable
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            rule.manual_required.join("<br>").replace('|', "\\|")
        ));
    }
    out
}

pub fn write_fixture_matrix_files(
    matrix: &RuleFixtureMatrix,
    output_dir: &Path,
) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|err| format!("{}: {err}", output_dir.display()))?;
    let json = serde_json::to_string_pretty(matrix).map_err(|err| err.to_string())?;
    fs::write(output_dir.join("rule-fixture-matrix.json"), json)
        .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    fs::write(
        output_dir.join("rule-fixture-matrix.md"),
        render_fixture_matrix_summary(matrix),
    )
    .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    Ok(())
}

pub fn compare_fixture_matrix(
    catalog: &UpstreamCatalog,
    matrix: &RuleFixtureMatrix,
) -> RuleFixtureDriftReport {
    let upstream_rules = catalog
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let matrix_rules = matrix
        .rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();

    for rule_id in upstream_rules.keys() {
        if !matrix_rules.contains_key(rule_id) {
            items.push(RuleFixtureDriftItem {
                rule_id: (*rule_id).to_string(),
                drift_type: RuleFixtureDriftType::MissingFixture,
                message: "upstream rule has no fixture matrix entry".to_string(),
            });
        }
    }

    for entry in &matrix.rules {
        if !upstream_rules.contains_key(entry.rule_id.as_str()) {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::StaleFixture,
                message: "fixture matrix entry is not present in upstream docs".to_string(),
            });
        }
        if !entry.manual_required.is_empty() {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::ManualRequired,
                message: entry.manual_required.join("; "),
            });
        }
    }

    let mut summary = RuleFixtureDriftSummary {
        upstream_rules: upstream_rules.len(),
        matrix_rules: matrix_rules.len(),
        ..RuleFixtureDriftSummary::default()
    };
    for item in &items {
        match item.drift_type {
            RuleFixtureDriftType::MissingFixture => summary.missing_fixtures += 1,
            RuleFixtureDriftType::StaleFixture => summary.stale_fixtures += 1,
            RuleFixtureDriftType::ManualRequired => summary.manual_required += 1,
        }
    }

    RuleFixtureDriftReport {
        upstream_source: catalog.source.clone(),
        summary,
        items,
    }
}

pub fn compare_with_local(catalog: &UpstreamCatalog) -> DriftReport {
    let local_rules = local_rule_map();
    let upstream_rules = catalog
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();

    for upstream in &catalog.rules {
        let Some(local) = local_rules.get(upstream.id.as_str()) else {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::MissingLocalRule,
                message: "upstream rule is not present in local catalog".to_string(),
            });
            continue;
        };

        compare_rule_properties(upstream, local, &mut items);
        if normalize_summary(&upstream.summary) != normalize_summary(local.description) {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::SummaryMismatch,
                message: format!(
                    "summary differs: upstream {:?}, local {:?}",
                    upstream.summary, local.description
                ),
            });
        }
    }

    for local_id in local_rules.keys() {
        if !upstream_rules.contains_key(local_id) {
            items.push(DriftItem {
                rule_id: (*local_id).to_string(),
                drift_type: DriftType::RemovedUpstreamRule,
                message: "local rule is not present in upstream docs".to_string(),
            });
        }
    }

    let mut summary = DriftSummary {
        upstream_rules: upstream_rules.len(),
        local_rules: local_rules.len(),
        ..DriftSummary::default()
    };
    for item in &items {
        match item.drift_type {
            DriftType::MissingLocalRule => summary.missing += 1,
            DriftType::RemovedUpstreamRule => summary.removed += 1,
            DriftType::DeprecatedUpstreamRule => summary.deprecated += 1,
            DriftType::UnknownNeedsReview => summary.unknown_needs_review += 1,
            _ => summary.mismatches += 1,
        }
    }

    DriftReport {
        upstream_source: catalog.source.clone(),
        summary,
        items,
    }
}

pub fn render_markdown_summary(report: &DriftReport) -> String {
    let mut out = String::new();
    out.push_str("# Upstream Drift Report\n\n");
    out.push_str(&format!(
        "- upstream rules: {}\n",
        report.summary.upstream_rules
    ));
    out.push_str(&format!("- local rules: {}\n", report.summary.local_rules));
    out.push_str(&format!("- missing: {}\n", report.summary.missing));
    out.push_str(&format!("- removed: {}\n", report.summary.removed));
    out.push_str(&format!("- deprecated: {}\n", report.summary.deprecated));
    out.push_str(&format!("- mismatches: {}\n", report.summary.mismatches));
    out.push_str(&format!(
        "- unknown_needs_review: {}\n\n",
        report.summary.unknown_needs_review
    ));

    if report.items.is_empty() {
        out.push_str("No drift detected.\n");
        return out;
    }

    out.push_str("| Rule | Type | Message |\n");
    out.push_str("| --- | --- | --- |\n");
    for item in &report.items {
        out.push_str(&format!(
            "| {} | {:?} | {} |\n",
            item.rule_id,
            item.drift_type,
            item.message.replace('|', "\\|")
        ));
    }
    out
}

pub fn write_report_files(report: &DriftReport, output_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|err| format!("{}: {err}", output_dir.display()))?;
    let json = serde_json::to_string_pretty(report).map_err(|err| err.to_string())?;
    fs::write(output_dir.join("upstream-drift.json"), json)
        .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    fs::write(
        output_dir.join("upstream-drift.md"),
        render_markdown_summary(report),
    )
    .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    Ok(())
}

fn rule_fixture_entry(rule: &UpstreamRuleDocument) -> RuleFixtureEntry {
    let example_cases = rule
        .examples
        .iter()
        .enumerate()
        .map(|(index, example)| RuleFixtureCase {
            name: format!("official_example_{}", index + 1),
            source: example.content.clone(),
            expected: None,
        })
        .collect::<Vec<_>>();
    let config_valid = rule
        .properties
        .iter()
        .map(|property| RuleFixtureCase {
            name: format!("{}_valid", property.key),
            source: format!(
                "{{ \"{}\": {{ \"{}\": {} }} }}",
                rule.id,
                property.key,
                config_value_for(property)
            ),
            expected: None,
        })
        .collect::<Vec<_>>();
    let config_invalid = rule
        .properties
        .iter()
        .map(|property| RuleFixtureCase {
            name: format!("{}_invalid_type", property.key),
            source: format!("{{ \"{}\": {{ \"{}\": null }} }}", rule.id, property.key),
            expected: Some("invalid type".to_string()),
        })
        .collect::<Vec<_>>();
    let mut manual_required = Vec::new();
    if example_cases.is_empty() {
        manual_required
            .push("official document has no fenced examples for check fixtures".to_string());
    }
    if rule.fixable.unwrap_or(false) {
        manual_required.push("fix before/after fixture must be reviewed manually".to_string());
    }
    if rule.properties.is_empty() {
        manual_required.push("config fixture has no documented parameters".to_string());
    }

    RuleFixtureEntry {
        rule_id: rule.id.clone(),
        aliases: rule.aliases.clone(),
        tags: rule.tags.clone(),
        parameters: rule.properties.clone(),
        fixable: rule.fixable,
        check_pass: Vec::new(),
        check_fail: example_cases,
        fix: Vec::new(),
        config_valid,
        config_invalid,
        edge: Vec::new(),
        manual_required,
    }
}

fn config_value_for(property: &UpstreamRuleProperty) -> String {
    if let Some(default_value) = &property.default_value {
        return normalize_config_literal(default_value, &property.value_type);
    }
    match property.value_type.as_str() {
        "boolean" => "true".to_string(),
        "integer" => "1".to_string(),
        "array" => "[]".to_string(),
        _ => "\"value\"".to_string(),
    }
}

fn normalize_config_literal(value: &str, value_type: &str) -> String {
    let value = value.trim();
    let starts_with_number = value.chars().next().is_some_and(|ch| ch.is_ascii_digit());
    let is_json_literal =
        value.starts_with('[') || starts_with_number || matches!(value, "true" | "false");
    match value_type {
        "boolean" | "integer" | "array" if is_json_literal => value.to_string(),
        _ if value.starts_with('"') => value.to_string(),
        _ => format!("\"{}\"", value.replace('"', "\\\"")),
    }
}

fn compare_rule_properties(
    upstream: &UpstreamRuleDocument,
    local: &crate::rules::markdown::OfficialRuleMeta,
    items: &mut Vec<DriftItem>,
) {
    let upstream_props = upstream
        .properties
        .iter()
        .map(|property| (property.key.as_str(), property))
        .collect::<BTreeMap<_, _>>();
    let local_props = local
        .properties
        .iter()
        .map(|property| (property.key, property))
        .collect::<BTreeMap<_, _>>();

    for (key, upstream_property) in &upstream_props {
        let Some(local_property) = local_props.get(key) else {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyMissingLocal,
                message: format!("property `{key}` is present upstream but missing locally"),
            });
            continue;
        };

        let local_type = property_type_name(local_property.prop_type);
        if upstream_property.value_type != local_type {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyTypeMismatch,
                message: format!(
                    "property `{key}` type differs: upstream {}, local {}",
                    upstream_property.value_type, local_type
                ),
            });
        }
        if let Some(default_value) = &upstream_property.default_value {
            if normalize_default(default_value) != normalize_default(local_property.default_value) {
                items.push(DriftItem {
                    rule_id: upstream.id.clone(),
                    drift_type: DriftType::PropertyDefaultMismatch,
                    message: format!(
                        "property `{key}` default differs: upstream {}, local {}",
                        default_value, local_property.default_value
                    ),
                });
            }
        }
    }

    for key in local_props.keys() {
        if !upstream_props.contains_key(key) {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyMissingUpstream,
                message: format!("property `{key}` is present locally but missing upstream"),
            });
        }
    }
}

fn local_rule_map() -> BTreeMap<&'static str, crate::rules::markdown::OfficialRuleMeta> {
    let mut map = BTreeMap::new();
    for rule in crate::rules::markdown::MarkdownLinterOps::user_configurable_rules() {
        if let Some(meta) = rule.official_meta() {
            map.insert(meta.code, meta);
        }
    }
    for rule in local_metadata_stubs() {
        if let Some(meta) = rule.official_meta() {
            let replace = map
                .get(meta.code)
                .map(|current| meta.properties.len() > current.properties.len())
                .unwrap_or(true);
            if replace {
                map.insert(meta.code, meta);
            }
        }
    }
    map
}

fn local_metadata_stubs() -> Vec<Box<dyn crate::rules::markdown::MarkdownRule>> {
    use crate::rules::markdown::stubs::*;
    vec![
        Box::new(RuleMD001),
        Box::new(RuleMD003),
        Box::new(RuleMD004),
        Box::new(RuleMD005),
        Box::new(RuleMD007),
        Box::new(RuleMD011),
        Box::new(RuleMD012),
        Box::new(RuleMD013),
        Box::new(RuleMD014),
        Box::new(RuleMD020),
        Box::new(RuleMD021),
        Box::new(RuleMD022),
        Box::new(RuleMD023),
        Box::new(RuleMD024),
        Box::new(RuleMD025),
        Box::new(RuleMD026),
        Box::new(RuleMD027),
        Box::new(RuleMD028),
        Box::new(RuleMD029),
        Box::new(RuleMD030),
        Box::new(RuleMD031),
        Box::new(RuleMD032),
        Box::new(RuleMD033),
        Box::new(RuleMD034),
        Box::new(RuleMD035),
        Box::new(RuleMD036),
        Box::new(RuleMD040),
        Box::new(RuleMD041),
        Box::new(RuleMD042),
        Box::new(RuleMD043),
        Box::new(RuleMD044),
        Box::new(RuleMD045),
        Box::new(RuleMD046),
        Box::new(RuleMD047),
        Box::new(RuleMD048),
        Box::new(RuleMD049),
        Box::new(RuleMD050),
        Box::new(RuleMD051),
        Box::new(RuleMD052),
        Box::new(RuleMD053),
        Box::new(RuleMD054),
        Box::new(RuleMD055),
        Box::new(RuleMD056),
        Box::new(RuleMD058),
        Box::new(RuleMD059),
        Box::new(RuleMD060),
    ]
}

fn parse_fenced_examples(source: &str) -> Vec<UpstreamRuleExample> {
    let mut examples = Vec::new();
    let mut in_fence = false;
    let mut language = None;
    let mut content = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("```") {
            if in_fence {
                examples.push(UpstreamRuleExample {
                    language: language.take(),
                    content: content.trim_end_matches('\n').to_string(),
                });
                content.clear();
                in_fence = false;
            } else {
                let lang = rest.trim();
                language = if lang.is_empty() {
                    None
                } else {
                    Some(lang.to_string())
                };
                in_fence = true;
            }
            continue;
        }
        if in_fence {
            content.push_str(line);
            content.push('\n');
        }
    }
    examples
}

fn parse_header(header: &str) -> Result<(String, String), String> {
    let Some(rest) = header.strip_prefix("# `") else {
        return Err(format!("invalid rule header: {header}"));
    };
    let Some((id, after_id)) = rest.split_once("` - ") else {
        return Err(format!("invalid rule header: {header}"));
    };
    Ok((id.to_string(), after_id.trim().to_string()))
}

fn parse_backtick_list(value: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut remaining = value;
    while let Some(start) = remaining.find('`') {
        let after_start = &remaining[start + 1..];
        let Some(end) = after_start.find('`') else {
            break;
        };
        values.push(after_start[..end].to_string());
        remaining = &after_start[end + 1..];
    }
    values
}

fn parse_parameter_block(lines: &[String]) -> Vec<UpstreamRuleProperty> {
    let mut bullets = Vec::new();
    for line in lines {
        if line.starts_with("- `") {
            bullets.push(line.clone());
        } else if let Some(last) = bullets.last_mut() {
            last.push(' ');
            last.push_str(line.trim());
        }
    }

    bullets
        .iter()
        .filter_map(|line| parse_parameter_line(line))
        .collect()
}

fn parse_parameter_line(line: &str) -> Option<UpstreamRuleProperty> {
    let rest = line.strip_prefix("- `")?;
    let (key, after_key) = rest.split_once("`:")?;
    let value_type = extract_parenthesized(after_key)
        .and_then(|inside| inside.split(',').next().map(str::trim).map(normalize_type))
        .unwrap_or_else(|| "unknown".to_string());
    let default_value = after_key
        .split("default `")
        .nth(1)
        .and_then(|rest| rest.split('`').next())
        .map(str::to_string);
    let values = after_key
        .split("values ")
        .nth(1)
        .map(parse_values)
        .unwrap_or_default();
    Some(UpstreamRuleProperty {
        key: key.to_string(),
        value_type,
        default_value,
        values,
    })
}

fn extract_parenthesized(value: &str) -> Option<&str> {
    let start = value.rfind('(')?;
    let end = value[start + 1..].find(')')?;
    Some(&value[start + 1..start + 1 + end])
}

fn parse_values(value: &str) -> Vec<String> {
    let backtick_values = parse_backtick_list(value);
    if !backtick_values.is_empty() {
        return backtick_values;
    }

    value
        .split('/')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn property_type_name(prop_type: RulePropertyType) -> &'static str {
    match prop_type {
        RulePropertyType::Boolean => "boolean",
        RulePropertyType::Number => "integer",
        RulePropertyType::String => "string",
        RulePropertyType::StringArray => "array",
        RulePropertyType::Enum(_) => "string",
    }
}

fn normalize_type(value: &str) -> String {
    let value = value.trim().trim_matches('`');
    match value {
        "number" => "integer".to_string(),
        "string array" | "string[]" => "array".to_string(),
        other => other.to_string(),
    }
}

fn normalize_default(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .replace(' ', "")
        .replace("\\\"", "\"")
}

fn normalize_summary(value: &str) -> String {
    value.trim().trim_end_matches('.').to_ascii_lowercase()
}

fn is_rule_doc(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.len() == 8
        && file_name.starts_with("md")
        && file_name.ends_with(".md")
        && file_name[2..5].chars().all(|ch| ch.is_ascii_digit())
}

pub fn assert_no_unknown_drift(report: &DriftReport, allowed: &BTreeSet<(String, DriftType)>) {
    let unknown = report
        .items
        .iter()
        .filter(|item| !allowed.contains(&(item.rule_id.clone(), item.drift_type)))
        .collect::<Vec<_>>();
    assert!(
        unknown.is_empty(),
        "unknown upstream drift detected:\n{}",
        render_markdown_summary(report)
    );
}

pub fn known_current_drift_allowlist() -> BTreeSet<(String, DriftType)> {
    [
        ("MD003", DriftType::SummaryMismatch),
        ("MD005", DriftType::SummaryMismatch),
        ("MD007", DriftType::SummaryMismatch),
        ("MD013", DriftType::SummaryMismatch),
        ("MD014", DriftType::SummaryMismatch),
        ("MD018", DriftType::SummaryMismatch),
        ("MD019", DriftType::SummaryMismatch),
        ("MD020", DriftType::SummaryMismatch),
        ("MD021", DriftType::SummaryMismatch),
        ("MD022", DriftType::PropertyTypeMismatch),
        ("MD034", DriftType::SummaryMismatch),
        ("MD043", DriftType::SummaryMismatch),
        ("MD044", DriftType::SummaryMismatch),
        ("MD051", DriftType::SummaryMismatch),
        ("MD052", DriftType::SummaryMismatch),
        ("MD053", DriftType::SummaryMismatch),
        ("MD054", DriftType::SummaryMismatch),
        ("MD056", DriftType::SummaryMismatch),
        ("MD058", DriftType::SummaryMismatch),
        ("MD059", DriftType::SummaryMismatch),
    ]
    .into_iter()
    .map(|(rule_id, drift_type)| (rule_id.to_string(), drift_type))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rule_document_parameters() {
        let document = parse_rule_document(
            r#"# `MD060` - Table column style

Tags: `table`

Aliases: `table-column-style`

Parameters:

- `aligned_delimiter`: Aligned delimiter columns (`boolean`, default `false`)
- `style`: Table column style (`string`, default `any`, values `aligned` /
  `any` / `compact` / `tight`)
"#,
        )
        .expect("document should parse");

        assert_eq!(document.id, "MD060");
        assert_eq!(document.name, "table-column-style");
        assert_eq!(document.tags, vec!["table"]);
        assert_eq!(document.properties.len(), 2);
        assert_eq!(
            document.properties[1].values,
            vec!["aligned", "any", "compact", "tight"]
        );
    }

    #[test]
    fn parses_rule_document_fenced_examples() {
        let document = parse_rule_document(
            r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

This is not a fixture.

```markdown
# Heading

text
```

```json
{ "MD999": false }
```
"#,
        )
        .expect("document should parse");

        assert_eq!(document.examples.len(), 2);
        assert_eq!(document.examples[0].language.as_deref(), Some("markdown"));
        assert_eq!(document.examples[0].content, "# Heading\n\ntext");
        assert_eq!(document.examples[1].language.as_deref(), Some("json"));
    }

    #[test]
    fn builds_fixture_matrix_with_required_schema_fields() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

Parameters:

- `enabled`: Enable example (`boolean`, default `true`)

Fixable: Some violations can be fixed by tooling.

```markdown
bad
```
"#,
            )
            .expect("document should parse")],
        };

        let matrix = build_fixture_matrix(&catalog);
        let json = serde_json::to_value(&matrix).expect("matrix should serialize");
        let rule = &matrix.rules[0];

        assert_eq!(matrix.summary.total_rules, 1);
        assert_eq!(matrix.summary.rules_with_examples, 1);
        assert_eq!(matrix.summary.rules_with_parameters, 1);
        assert_eq!(matrix.summary.rules_with_fix_metadata, 1);
        assert_eq!(rule.rule_id, "MD999");
        assert_eq!(rule.aliases, vec!["example-rule"]);
        assert_eq!(rule.tags, vec!["test"]);
        assert_eq!(rule.parameters[0].key, "enabled");
        assert_eq!(rule.fixable, Some(true));
        assert_eq!(rule.check_fail.len(), 1);
        assert_eq!(
            rule.config_valid[0].source,
            r#"{ "MD999": { "enabled": true } }"#
        );
        assert_eq!(
            rule.config_invalid[0].expected.as_deref(),
            Some("invalid type")
        );

        for key in [
            "rule_id",
            "aliases",
            "tags",
            "parameters",
            "fixable",
            "check_pass",
            "check_fail",
            "fix",
            "config_valid",
            "config_invalid",
            "edge",
            "manual_required",
        ] {
            assert!(
                json["rules"][0].get(key).is_some(),
                "missing schema key {key}"
            );
        }
    }

    #[test]
    fn fixture_matrix_reports_missing_stale_and_manual_required_items() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD001` - Heading increment

Tags: `headings`

Aliases: `heading-increment`
"#,
            )
            .expect("document should parse")],
        };
        let matrix = RuleFixtureMatrix {
            upstream_source: "test".to_string(),
            summary: RuleFixtureMatrixSummary::default(),
            rules: vec![RuleFixtureEntry {
                rule_id: "MD999".to_string(),
                aliases: Vec::new(),
                tags: Vec::new(),
                parameters: Vec::new(),
                fixable: None,
                check_pass: Vec::new(),
                check_fail: Vec::new(),
                fix: Vec::new(),
                config_valid: Vec::new(),
                config_invalid: Vec::new(),
                edge: Vec::new(),
                manual_required: vec!["review needed".to_string()],
            }],
        };

        let drift = compare_fixture_matrix(&catalog, &matrix);

        assert_eq!(drift.summary.missing_fixtures, 1);
        assert_eq!(drift.summary.stale_fixtures, 1);
        assert_eq!(drift.summary.manual_required, 1);
        assert!(drift
            .items
            .iter()
            .any(|item| item.drift_type == RuleFixtureDriftType::MissingFixture));
        assert!(drift
            .items
            .iter()
            .any(|item| item.drift_type == RuleFixtureDriftType::StaleFixture));
    }

    #[test]
    fn renders_and_writes_fixture_matrix_reports() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

```markdown
bad
```
"#,
            )
            .expect("document should parse")],
        };
        let matrix = build_fixture_matrix(&catalog);
        let markdown = render_fixture_matrix_summary(&matrix);
        let dir = std::env::temp_dir().join(format!(
            "katana-markdown-linter-fixture-matrix-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);

        write_fixture_matrix_files(&matrix, &dir).expect("matrix files should be written");

        assert!(markdown.contains("Rule Fixture Matrix"));
        assert!(markdown.contains("MD999"));
        assert!(dir.join("rule-fixture-matrix.json").exists());
        assert!(dir.join("rule-fixture-matrix.md").exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn renders_markdown_summary() {
        let report = DriftReport {
            upstream_source: "test".to_string(),
            summary: DriftSummary {
                upstream_rules: 1,
                local_rules: 0,
                missing: 1,
                ..DriftSummary::default()
            },
            items: vec![DriftItem {
                rule_id: "MD999".to_string(),
                drift_type: DriftType::MissingLocalRule,
                message: "missing".to_string(),
            }],
        };

        let markdown = render_markdown_summary(&report);
        assert!(markdown.contains("MD999"));
        assert!(markdown.contains("missing: 1"));
    }

    #[test]
    fn writes_json_and_markdown_reports() {
        let report = DriftReport {
            upstream_source: "test".to_string(),
            summary: DriftSummary::default(),
            items: Vec::new(),
        };
        let dir = std::env::temp_dir().join(format!(
            "katana-markdown-linter-drift-report-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);

        write_report_files(&report, &dir).expect("report files should be written");

        assert!(dir.join("upstream-drift.json").exists());
        assert!(dir.join("upstream-drift.md").exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[ignore = "requires KML_UPSTREAM_MARKDOWNLINT_DOC_DIR pointing at DavidAnson/markdownlint/doc"]
    fn upstream_default_branch_drift_has_no_unknown_items() {
        let doc_dir = std::env::var("KML_UPSTREAM_MARKDOWNLINT_DOC_DIR")
            .expect("KML_UPSTREAM_MARKDOWNLINT_DOC_DIR must be set");
        let catalog = load_catalog_from_dir(Path::new(&doc_dir)).expect("catalog should load");
        let report = compare_with_local(&catalog);
        assert_no_unknown_drift(&report, &known_current_drift_allowlist());
    }
}
