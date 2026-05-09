use super::*;

#[test]
fn load_supports_jsonc_comments_and_trailing_commas() {
    let path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-jsonc-{}.jsonc",
        std::process::id()
    ));
    fs::write(
        &path,
        r#"{
  /* default config */
  "default": true,
  "MD001": {
    "front_matter_title": "^title[:=]$",
  },
}"#,
    )
    .expect("test config should write");

    let config = MarkdownLintConfig::load(&path).expect("jsonc should load");
    assert_eq!(
        config.get_rule_property("MD001", "front_matter_title"),
        Some(&json!("^title[:=]$"))
    );

    let _ = fs::remove_file(&path);
}

#[test]
fn parse_config_text_keeps_strings_while_stripping_comments() {
    let parsed = crate::rules::markdown::config::jsonc::parse_config_text(
        "{\n  \"default\": true,\n  \"url\": \"https://example.com//not-comment\",\n  /* block comment */\n  \"escaped\": \"quote: \\\"//not-comment\\\"\",\r\n  // crlf comment\r\n  \"items\": [\"a\",],\n}",
    )
    .expect("jsonc should parse");

    assert_eq!(parsed["url"], json!("https://example.com//not-comment"));
    assert_eq!(parsed["escaped"], json!("quote: \"//not-comment\""));
    assert_eq!(parsed["items"], json!(["a"]));
}

#[test]
fn create_default_file_writes_default_when_missing() {
    let path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-default-{}.json",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);

    let config =
        MarkdownLintConfig::create_default_file(&path).expect("default config should be created");
    assert_eq!(config.raw, json!({ "default": true }));

    let loaded = MarkdownLintConfig::load(&path).expect("created file should load");
    assert_eq!(loaded.raw, config.raw);

    let _ = fs::remove_file(&path);
}

#[test]
fn create_default_file_loads_existing_config() {
    let path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-existing-{}.json",
        std::process::id()
    ));
    fs::write(&path, r#"{ "default": false }"#).expect("test config should write");

    let config =
        MarkdownLintConfig::create_default_file(&path).expect("existing config should be loaded");
    assert_eq!(config.raw, json!({ "default": false }));

    let _ = fs::remove_file(&path);
}
