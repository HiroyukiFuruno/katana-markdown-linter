use super::{assert_no_violations, workspace_root};

#[test]
fn ast_linter_supported_locales_match_katana_language_set() {
    let actual = katana_markdown_linter::supported_locales()
        .iter()
        .map(|locale| locale.code())
        .collect::<Vec<_>>();
    let expected = [
        "en", "ja", "zh-CN", "zh-TW", "ko", "pt", "fr", "de", "es", "it",
    ];
    let violations = if actual == expected {
        Vec::new()
    } else {
        vec![format!(
            "i18n: supported locales differ; actual={actual:?}, expected={expected:?}"
        )]
    };

    assert_no_violations("i18n-supported-locales", violations);
}

#[test]
fn ast_linter_localized_rule_docs_exist_and_are_not_english_copies() {
    let root = workspace_root();
    let mut violations = Vec::new();
    for rule in katana_markdown_linter::available_rules() {
        let english_path = root
            .join("upstream_docs")
            .join(format!("{}.md", rule.id.to_lowercase()));
        let english = read(&english_path, &mut violations);
        for locale in katana_markdown_linter::supported_locales() {
            if *locale == katana_markdown_linter::Locale::En {
                continue;
            }

            let path = root
                .join("upstream_docs")
                .join(locale.code())
                .join(format!("{}.md", rule.id.to_lowercase()));
            let localized = read(&path, &mut violations);
            if localized.is_empty() {
                continue;
            }
            if localized == english {
                violations.push(format!(
                    "{}: localized document is an English copy",
                    path.display()
                ));
            }
            if prose_without_code(&localized) == prose_without_code(&english) {
                violations.push(format!(
                    "{}: localized prose matches English prose",
                    path.display()
                ));
            }
            if !has_expected_script(*locale, &localized) {
                violations.push(format!(
                    "{}: localized document lacks expected script for {}",
                    path.display(),
                    locale.code()
                ));
            }
        }
    }

    assert_no_violations("i18n-rule-doc-coverage", violations);
}

fn read(path: &std::path::Path, violations: &mut Vec<String>) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            violations.push(format!("{}: {err}", path.display()));
            String::new()
        }
    }
}

fn prose_without_code(content: &str) -> String {
    let mut prose = String::new();
    let mut in_code = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if !in_code {
            prose.push_str(line.trim());
            prose.push('\n');
        }
    }
    prose
}

fn has_expected_script(locale: katana_markdown_linter::Locale, content: &str) -> bool {
    match locale {
        katana_markdown_linter::Locale::Ja => content.chars().any(is_japanese),
        katana_markdown_linter::Locale::ZhCn | katana_markdown_linter::Locale::ZhTw => {
            content.chars().any(is_cjk)
        }
        katana_markdown_linter::Locale::Ko => content.chars().any(is_hangul),
        _ => true,
    }
}

fn is_japanese(ch: char) -> bool {
    ('\u{3040}'..='\u{30ff}').contains(&ch) || is_cjk(ch)
}

fn is_cjk(ch: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

fn is_hangul(ch: char) -> bool {
    ('\u{ac00}'..='\u{d7af}').contains(&ch)
}
