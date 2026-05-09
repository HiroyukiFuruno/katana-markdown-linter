use katana_markdown_linter::{upstream, I18nRuleDescriptionService, Locale};

#[test]
fn localized_descriptions_cover_non_japanese_locale() {
    let english = "Heading style should be consistent";
    let french = I18nRuleDescriptionService::localized_rule_description("MD003", english, "fr-FR");

    assert_eq!(french, "Conservez un style de titre cohérent");
    assert_ne!(french, english);
}

#[test]
fn localized_rule_documentation_uses_requested_supported_locale() {
    let content = upstream::UpstreamDocumentService::get_rule_documentation("MD013", Locale::Fr)
        .expect("French MD013 documentation should exist");

    assert!(content.contains("# `MD013` - La longueur de ligne dépasse la limite"));
    assert!(content.contains("## Vue d'ensemble"));
    assert!(!content.contains("# `MD013` - Line length"));
}

#[test]
fn unsupported_documentation_locale_resolves_to_english_before_lookup() {
    let locale = Locale::resolve_code("sv-SE");
    let content = upstream::UpstreamDocumentService::get_rule_documentation("MD013", locale)
        .expect("English fallback documentation should exist");

    assert_eq!(locale, Locale::En);
    assert!(content.contains("# `MD013` - Line length"));
}
