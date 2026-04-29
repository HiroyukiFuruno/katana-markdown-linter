#[path = "i18n_catalog.rs"]
mod i18n_catalog;
#[path = "i18n_doc.rs"]
mod i18n_doc;
#[path = "i18n_text.rs"]
mod i18n_text;

use super::assert_no_violations;
use i18n_catalog::{
    active_rule_ids, compare_sets, locale_file_codes, read_catalog, read_languages,
    validate_languages, EXPECTED_CODES,
};

#[test]
fn ast_linter_supported_locales_match_katana_language_set() {
    let actual = katana_markdown_linter::supported_locales()
        .iter()
        .map(|locale| locale.code())
        .collect::<Vec<_>>();
    let violations = if actual == EXPECTED_CODES {
        Vec::new()
    } else {
        vec![format!(
            "i18n: supported locales differ; actual={actual:?}, expected={EXPECTED_CODES:?}"
        )]
    };

    assert_no_violations("i18n-supported-locales", violations);
}

#[test]
fn ast_linter_locale_json_catalog_is_consistent() {
    let locale_dir = i18n_catalog::locale_dir();
    let mut violations = Vec::new();
    let languages = read_languages(&locale_dir, &mut violations);
    let declared = validate_languages(&languages, &mut violations);
    let files = locale_file_codes(&locale_dir, &mut violations);
    compare_sets("languages.json", &declared, &files, &mut violations);

    let active_rules = active_rule_ids();
    let english = read_catalog(&locale_dir, "en", &mut violations);
    validate_english_canonical_descriptions(english.as_ref(), &mut violations);

    for code in declared {
        let Some(catalog) = read_catalog(&locale_dir, &code, &mut violations) else {
            continue;
        };
        compare_sets(
            &format!("{code}.json rule_descriptions"),
            &active_rules,
            &catalog.rule_descriptions.keys().cloned().collect(),
            &mut violations,
        );
        i18n_catalog::validate_rule_doc_catalog(&code, &catalog.rule_doc, &mut violations);
        validate_description_values(&code, &catalog, english.as_ref(), &mut violations);
    }

    assert_no_violations("i18n-locale-json-catalog", violations);
}

#[test]
fn ast_linter_localized_rule_docs_exist_and_match_locale_json() {
    let root = super::workspace_root();
    let locale_dir = i18n_catalog::locale_dir();
    let mut violations = Vec::new();
    let languages = read_languages(&locale_dir, &mut violations);

    for language in languages.iter().filter(|language| language.code != "en") {
        let Some(catalog) = read_catalog(&locale_dir, &language.code, &mut violations) else {
            continue;
        };
        for rule in katana_markdown_linter::available_rules() {
            let english_path = root
                .join("upstream_docs")
                .join(format!("{}.md", rule.id.to_lowercase()));
            let localized_path = root
                .join("upstream_docs")
                .join(&language.code)
                .join(format!("{}.md", rule.id.to_lowercase()));
            let english = i18n_text::read_to_string(&english_path, &mut violations);
            let localized = i18n_text::read_to_string(&localized_path, &mut violations);
            if !localized.is_empty() {
                i18n_doc::validate_rule_doc_content(
                    &language.code,
                    &localized_path,
                    &rule.id,
                    &localized,
                    &english,
                    &catalog,
                    &mut violations,
                );
            }
        }
    }

    assert_no_violations("i18n-rule-doc-coverage", violations);
}

fn validate_english_canonical_descriptions(
    english: Option<&i18n_catalog::LocaleCatalog>,
    violations: &mut Vec<String>,
) {
    for rule in katana_markdown_linter::available_rules() {
        if english
            .and_then(|catalog| catalog.rule_descriptions.get(&rule.id))
            .is_some_and(|description| description != &rule.description)
        {
            violations.push(format!(
                "src/i18n/locales/en.json: {} description does not match canonical rule metadata",
                rule.id
            ));
        }
    }
}

fn validate_description_values(
    code: &str,
    catalog: &i18n_catalog::LocaleCatalog,
    english: Option<&i18n_catalog::LocaleCatalog>,
    violations: &mut Vec<String>,
) {
    for (rule_id, description) in &catalog.rule_descriptions {
        if description.trim().is_empty() {
            violations.push(format!("{code}.json: {rule_id} description is empty"));
        }
        if code != "en"
            && english
                .and_then(|catalog| catalog.rule_descriptions.get(rule_id))
                .is_some_and(|english| english.trim() == description.trim())
        {
            violations.push(format!(
                "{code}.json: {rule_id} description is an English copy"
            ));
        }
        if !i18n_text::has_expected_script(code, description) {
            violations.push(format!(
                "{code}.json: {rule_id} description lacks expected script"
            ));
        }
    }
}
