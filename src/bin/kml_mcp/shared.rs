use crate::model::{Diagnostic, RuleMetadata};
use katana_markdown_linter::{rule_catalog, LintResult, Locale};

pub(crate) fn request_locale(locale: Option<&str>) -> Locale {
    locale
        .map(katana_markdown_linter::resolve_locale_code)
        .unwrap_or(Locale::En)
}

pub(crate) fn diagnostics(results: Vec<LintResult>, locale: Locale) -> Vec<Diagnostic> {
    results
        .into_iter()
        .map(|result| Diagnostic::from_result(result, locale))
        .collect()
}

pub(crate) fn catalog_rules(locale: Locale) -> Vec<RuleMetadata> {
    let catalog = rule_catalog();
    catalog
        .active
        .into_iter()
        .map(|rule| RuleMetadata::from_entry(rule, "active", locale))
        .chain(
            catalog
                .deprecated
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "deprecated", locale)),
        )
        .chain(
            catalog
                .removed
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "removed", locale)),
        )
        .collect()
}
