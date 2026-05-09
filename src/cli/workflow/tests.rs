use super::CliWorkflow;
use crate::cli::args::{Cli, Command, ConfigCommand, OutputFormat};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

mod format_config;

fn test_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be available")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "katana-markdown-linter-cli-{name}-{}-{nanos}",
        std::process::id()
    ))
}

#[test]
fn renders_rule_list_and_detail_contract() {
    use crate::i18n::Locale;
    let list = super::config_cmd::render_rule(None, OutputFormat::Text, Locale::En)
        .expect("rule list should render");
    assert!(list.contains("MD013 line-length"));
    assert!(list.contains("Line length"));

    let detail = super::config_cmd::render_rule(Some("MD013"), OutputFormat::Text, Locale::En)
        .expect("rule detail should render");
    assert!(detail.contains("MD013 line-length"));
    assert!(detail.contains("Line length"));
    assert!(detail.contains("markdownlint"));

    let json = super::config_cmd::render_rule(Some("MD013"), OutputFormat::Json, Locale::En)
        .expect("rule detail json should render");
    let value: serde_json::Value = serde_json::from_str(&json).expect("rule detail should be json");
    assert_eq!(value["id"], "MD013");
    assert_eq!(value["locale"], "en");
    assert_eq!(value["description"], value["english_description"]);
}

#[test]
fn rule_output_uses_selected_japanese_locale() {
    use crate::i18n::Locale;
    let list = super::config_cmd::render_rule(None, OutputFormat::Text, Locale::Ja)
        .expect("rule list should render");
    assert!(list.contains("MD003 heading-style - 見出しのスタイルを統一してください"));

    let json = super::config_cmd::render_rule(Some("MD003"), OutputFormat::Json, Locale::Ja)
        .expect("rule detail json should render");
    let value: serde_json::Value = serde_json::from_str(&json).expect("rule detail should be json");
    assert_eq!(value["locale"], "ja");
    assert_eq!(value["description"], "見出しのスタイルを統一してください");
    assert_eq!(value["english_description"], "Heading style");
}

#[test]
fn rule_output_uses_selected_non_japanese_locale() {
    use crate::i18n::Locale;
    let list = super::config_cmd::render_rule(None, OutputFormat::Text, Locale::Fr)
        .expect("rule list should render");
    assert!(list.contains("MD003 heading-style - Conservez un style de titre cohérent"));

    let json = super::config_cmd::render_rule(Some("MD003"), OutputFormat::Json, Locale::Fr)
        .expect("rule detail json should render");
    let value: serde_json::Value = serde_json::from_str(&json).expect("rule detail should be json");
    assert_eq!(value["locale"], "fr");
    assert_eq!(value["description"], "Conservez un style de titre cohérent");
    assert_eq!(value["english_description"], "Heading style");
}

#[test]
fn renders_config_file_and_get_contract() {
    let dir = test_dir("config-render");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let config = dir.join(".markdownlint.json");
    fs::write(&config, "{ \"default\": false }\n").expect("config should be written");
    let cli = Cli {
        command: Command::Config(ConfigCommand::Get),
        config: Some(config.clone()),
        format: OutputFormat::Json,
        ..Cli::default()
    };

    let file = super::config_cmd::render_config(ConfigCommand::File, &cli)
        .expect("config file should render as json");
    let file_json: serde_json::Value =
        serde_json::from_str(&file).expect("config file should be json");
    assert_eq!(file_json["path"], config.display().to_string());

    let get = super::config_cmd::render_config(ConfigCommand::Get, &cli)
        .expect("config get should render");
    let get_json: serde_json::Value =
        serde_json::from_str(&get).expect("config get should be json");
    assert_eq!(get_json, serde_json::json!({ "default": false }));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn check_returns_one_when_violations_exist() {
    let dir = test_dir("check-violation");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    fs::write(&file, "#Title\n").expect("test file should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Check,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("check should run");

    assert_eq!(exit, 1);
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn fix_returns_zero_when_fixable_violations_are_resolved() {
    let dir = test_dir("fix-violation");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("test file should be written");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config),
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("fix should run");

    assert_eq!(exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("fixed file should be readable"),
        "# Title\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn default_fix_does_not_apply_unsafe_candidates() {
    let dir = test_dir("unsafe-default");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
    fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
        .expect("config should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config),
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("fix should run");

    assert_eq!(exit, 1);
    assert_eq!(
        fs::read_to_string(&file).expect("file should be readable"),
        "**Important**\n\nText\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unsafe_fix_requires_yes_in_non_interactive_mode() {
    let dir = test_dir("unsafe-needs-yes");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
    fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
        .expect("config should be written");

    let result = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config),
        inputs: vec![file.display().to_string()],
        unsafe_fixes: true,
        ..Cli::default()
    });

    assert!(result
        .expect_err("unsafe non-interactive run should fail")
        .contains("--yes"));
    assert_eq!(
        fs::read_to_string(&file).expect("file should be readable"),
        "**Important**\n\nText\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unsafe_yes_applies_unsafe_fix() {
    let dir = test_dir("unsafe-yes");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
    fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
        .expect("config should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config),
        inputs: vec![file.display().to_string()],
        unsafe_fixes: true,
        yes: true,
        ..Cli::default()
    })
    .expect("unsafe fix should run with --yes");

    assert_eq!(exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("file should be readable"),
        "# Important\n\nText\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unsafe_confirmation_accepts_only_explicit_yes() {
    let mut yes = std::io::Cursor::new(b"Y".to_vec());
    let mut output = Vec::new();
    assert!(
        super::config_cmd::prompt_unsafe_confirmation(&mut yes, &mut output)
            .expect("prompt should run")
    );

    let mut default_yes = std::io::Cursor::new(b"\n".to_vec());
    let mut output = Vec::new();
    assert!(
        super::config_cmd::prompt_unsafe_confirmation(&mut default_yes, &mut output)
            .expect("prompt should run")
    );

    let mut no = std::io::Cursor::new(b"n".to_vec());
    let mut output = Vec::new();
    assert!(
        !super::config_cmd::prompt_unsafe_confirmation(&mut no, &mut output)
            .expect("prompt should run")
    );

    let mut eof = std::io::Cursor::new(Vec::<u8>::new());
    let mut output = Vec::new();
    assert!(
        !super::config_cmd::prompt_unsafe_confirmation(&mut eof, &mut output)
            .expect("prompt should run")
    );
}

#[test]
fn check_fix_applies_fixes_and_returns_zero_when_resolved() {
    let dir = test_dir("check-fix");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("test file should be written");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Check,
        config: Some(config),
        inputs: vec![file.display().to_string()],
        check_fix: true,
        ..Cli::default()
    })
    .expect("check --fix should run");

    assert_eq!(exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("fixed file should be readable"),
        "# Title\n"
    );
    let _ = fs::remove_dir_all(dir);
}
