use super::{i18n_catalog, i18n_text};
use std::path::Path;

pub(crate) fn validate_rule_doc_content(
    code: &str,
    path: &Path,
    rule_id: &str,
    localized: &str,
    english: &str,
    catalog: &i18n_catalog::LocaleCatalog,
    violations: &mut Vec<String>,
) {
    if localized == english {
        violations.push(format!(
            "{}: localized document is an English copy",
            path.display()
        ));
    }
    if i18n_text::prose_without_code(localized) == i18n_text::prose_without_code(english) {
        violations.push(format!(
            "{}: localized prose matches English prose",
            path.display()
        ));
    }
    if !i18n_text::has_expected_script(code, localized) {
        violations.push(format!(
            "{}: localized document lacks expected script",
            path.display()
        ));
    }
    validate_title(path, rule_id, localized, catalog, violations);
    validate_metadata_labels(path, localized, catalog, violations);
    validate_generated_sections(path, localized, catalog, violations);
}

fn validate_title(
    path: &Path,
    rule_id: &str,
    localized: &str,
    catalog: &i18n_catalog::LocaleCatalog,
    violations: &mut Vec<String>,
) {
    if let Some(description) = catalog.rule_descriptions.get(rule_id) {
        let expected_title = format!("# `{rule_id}` - {description}");
        if catalog.rule_doc.generation != "generated" {
            if !localized
                .lines()
                .next()
                .is_some_and(|line| line.starts_with(&format!("# `{rule_id}` - ")))
            {
                violations.push(format!("{}: title is missing rule ID", path.display()));
            }
            return;
        }
        if localized
            .lines()
            .next()
            .is_none_or(|line| line != expected_title)
        {
            violations.push(format!(
                "{}: title does not match locale JSON",
                path.display()
            ));
        }
    }
}

fn validate_metadata_labels(
    path: &Path,
    localized: &str,
    catalog: &i18n_catalog::LocaleCatalog,
    violations: &mut Vec<String>,
) {
    let required_labels = if catalog.rule_doc.generation == "generated" {
        &["tags", "aliases", "parameters"][..]
    } else {
        &["tags", "aliases"][..]
    };
    for label_key in required_labels.iter().copied() {
        let label = &catalog.rule_doc.labels[label_key];
        if !localized.contains(&format!("{label}:")) {
            violations.push(format!(
                "{}: missing localized `{label_key}` label",
                path.display()
            ));
        }
    }
}

fn validate_generated_sections(
    path: &Path,
    localized: &str,
    catalog: &i18n_catalog::LocaleCatalog,
    violations: &mut Vec<String>,
) {
    if catalog.rule_doc.generation != "generated" {
        return;
    }
    for label_key in ["overview", "config", "examples", "rationale"] {
        let label = &catalog.rule_doc.labels[label_key];
        if !localized.contains(&format!("## {label}")) {
            violations.push(format!(
                "{}: missing generated `{label_key}` section",
                path.display()
            ));
        }
    }
}
