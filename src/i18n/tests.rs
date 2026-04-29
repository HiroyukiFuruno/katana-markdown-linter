use super::*;

#[test]
fn locale_parsing_accepts_primary_region_and_charset_values() {
    assert_eq!(Locale::parse("en"), Some(Locale::En));
    assert_eq!(Locale::parse("en-US"), Some(Locale::En));
    assert_eq!(Locale::parse("ja_JP.UTF-8"), Some(Locale::Ja));
    assert_eq!(Locale::parse("zh-CN"), Some(Locale::ZhCn));
    assert_eq!(Locale::parse("zh_TW.UTF-8"), Some(Locale::ZhTw));
    assert_eq!(Locale::parse("ko-KR"), Some(Locale::Ko));
    assert_eq!(Locale::parse("pt-BR"), Some(Locale::Pt));
    assert_eq!(Locale::parse("fr-FR"), Some(Locale::Fr));
    assert_eq!(Locale::parse("de-DE"), Some(Locale::De));
    assert_eq!(Locale::parse("es-ES"), Some(Locale::Es));
    assert_eq!(Locale::parse("it-IT"), Some(Locale::It));
    assert_eq!(Locale::parse("sv"), None);
}

#[test]
fn public_locale_resolvers_accept_ui_language_codes() {
    assert_eq!(resolve_locale_code("en"), Locale::En);
    assert_eq!(resolve_locale_code("EN_us.UTF-8"), Locale::En);
    assert_eq!(resolve_locale_code("ja"), Locale::Ja);
    assert_eq!(resolve_locale_code("zh-Hant"), Locale::ZhTw);
    assert_eq!(resolve_locale_code("pt-PT"), Locale::Pt);
    assert_eq!(resolve_locale_code(""), Locale::En);
    assert_eq!(resolve_locale_code("sv"), Locale::En);
    assert_eq!(resolve_locale_code_or("sv", Locale::Ja), Locale::Ja);
}

#[test]
fn explicit_locale_overrides_os_locale_and_unsupported_os_falls_back_to_english() {
    assert_eq!(
        Locale::resolve_with(Some("en"), |_| Some("ja_JP.UTF-8".to_string())),
        Ok(Locale::En)
    );
    assert_eq!(
        Locale::resolve_with(None, |_| Some("sv_SE.UTF-8".to_string())),
        Ok(Locale::En)
    );
}

#[test]
fn catalog_keys_match_between_supported_locales() {
    for locale in supported_locales() {
        assert_eq!(catalog_keys(Locale::En), catalog_keys(*locale));
    }
}

#[test]
fn missing_translation_key_falls_back_to_english_message() {
    assert_eq!(
        render_message(
            Locale::Ja,
            "unknown.message",
            &MessageParams::new(),
            "English fallback"
        ),
        "English fallback"
    );
}

#[test]
fn md001_message_params_extract_expected_and_actual_levels() {
    let params = diagnostic_message_params(
        "MD001",
        "heading-increment",
        "Heading levels should only increment by one level at a time [Expected: h2, Actual: h4]",
    );
    assert_eq!(params["expected"], "h2");
    assert_eq!(params["actual"], "h4");
}

#[test]
fn localized_rule_description_uses_catalog_without_diagnostic_params() {
    assert_eq!(
        localized_rule_description("MD003", "Heading style should be consistent", "ja-JP"),
        "見出しのスタイルを統一してください"
    );
    assert_eq!(
        localized_rule_description("MD003", "Heading style should be consistent", "fr"),
        "Conservez un style de titre cohérent"
    );
    assert_eq!(
        localized_rule_description("MD003", "Heading style should be consistent", "sv"),
        "Heading style should be consistent"
    );
    assert!(
        localized_rule_description("MD999", "Custom fallback", "ja").contains("Custom fallback")
    );
}

#[test]
fn config_error_messages_render_with_structured_params() {
    let mut params = MessageParams::new();
    params.insert("rule_id".to_string(), "MD013".to_string());
    params.insert("property".to_string(), "line_length".to_string());
    params.insert("expected".to_string(), "number".to_string());
    params.insert("actual".to_string(), "string".to_string());

    assert_eq!(
        render_message(
            Locale::Ja,
            "config.invalid_type",
            &params,
            "invalid rule property value"
        ),
        "MD013.line_length の型が不正です。期待値: number、実際: string"
    );
}

#[test]
fn active_rule_description_translation_status_is_explicit() {
    for locale in supported_locales() {
        assert!(has_rule_description_translation("MD003", *locale));
    }
    assert!(!has_rule_description_translation("MD999", Locale::Ja));
}
