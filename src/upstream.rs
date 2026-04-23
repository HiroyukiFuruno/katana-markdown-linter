use crate::rules::markdown::RulePropertyType;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpstreamRuleDocument {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub properties: Vec<UpstreamRuleProperty>,
    pub fixable: Option<bool>,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpstreamRuleProperty {
    pub key: String,
    pub value_type: String,
    pub default_value: Option<String>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpstreamCatalog {
    pub source: String,
    pub rules: Vec<UpstreamRuleDocument>,
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
    for rule in crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules() {
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
