use katana_markdown_linter::{FixSafety, LintOptions, MarkdownLintConfig, MarkdownLinter};
use pulldown_cmark::{Event, Parser, Tag};
use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
enum Meaning {
    Text(String),
    Structure(String),
}

fn meaning(source: &str) -> Vec<Meaning> {
    let mut result = Vec::new();
    for event in Parser::new(source) {
        match event {
            Event::Text(text) => match result.last_mut() {
                Some(Meaning::Text(previous)) => previous.push_str(&text),
                _ => result.push(Meaning::Text(text.into_string())),
            },
            other => result.push(Meaning::Structure(format!("{other:?}"))),
        }
    }
    result
}

#[test]
fn style_fixes_preserve_meaning_across_delimiter_context_combinations() {
    let contexts = ["", "word ", "*", "_", r"\*", r"\\*", "日本語「"];
    let payloads = [
        "word",
        "pre *nested* post",
        "pre _nested_ post",
        "pre **strong** post",
        "`*code*`",
        "[label](https://example.invalid)",
        "<span>text</span>",
    ];
    let mut changes = 0;
    for (rule, width) in [("MD049", 1), ("MD050", 2)] {
        for (source_marker, style) in [("*", "underscore"), ("_", "asterisk")] {
            let options = MarkdownLintConfig {
                raw: json!({"default": false, (rule): {"enabled": true, "style": style}}),
            }
            .to_lint_options();
            let delimiter = source_marker.repeat(width);
            for prefix in contexts {
                for suffix in contexts {
                    for payload in payloads {
                        let source = format!("{prefix}{delimiter}{payload}{delimiter}{suffix}\n");
                        let fixed = MarkdownLinter::fix(&source, &options)
                            .expect("style fixing should converge");
                        assert_eq!(
                            meaning(&source),
                            meaning(&fixed.content),
                            "{rule} {style}: {source:?} -> {:?}",
                            fixed.content
                        );
                        if fixed.content != source {
                            changes += 1;
                        }
                        let second = MarkdownLinter::fix(&fixed.content, &options)
                            .expect("second fixing should converge");
                        assert_eq!(fixed.content, second.content);
                    }
                }
            }
        }
    }
    assert!(changes > 0, "the corpus must exercise real style edits");
}

#[test]
fn implicit_reference_emphasis_is_not_safely_rewritten() {
    for (rule_id, source, destination) in [
        (
            "MD049",
            "[*foo*] [*foo*][] ![*foo*] ![*foo*][]\n\n[*foo*]: /reference\n",
            "/reference",
        ),
        (
            "MD050",
            "[**foo**] [**foo**][] ![**foo**] ![**foo**][]\n\n[**foo**]: /reference-strong\n",
            "/reference-strong",
        ),
    ] {
        let options = style_options(rule_id);
        let diagnostics = MarkdownLinter::lint(source, &options).expect("lint should run");
        assert_eq!(
            diagnostics
                .iter()
                .filter(|item| item.rule_id == rule_id)
                .count(),
            4
        );
        for diagnostic in diagnostics.iter().filter(|item| item.rule_id == rule_id) {
            if let Some(fix) = &diagnostic.fix {
                assert_ne!(
                    fix.safety,
                    FixSafety::Safe,
                    "{rule_id} must not offer a safe fix that changes an implicit reference label"
                );
            }
        }
        let fixed = MarkdownLinter::fix(source, &options).expect("fix should run");
        assert_eq!(
            fixed.content, source,
            "{rule_id} must preserve shortcut/collapsed links and images"
        );
        assert_eq!(
            destinations(source),
            vec![
                (false, destination.to_string()),
                (false, destination.to_string()),
                (true, destination.to_string()),
                (true, destination.to_string()),
            ],
            "all four implicit link/image references should resolve in the source"
        );
        assert_eq!(destinations(&fixed.content), destinations(source));
    }
}

#[test]
fn explicit_reference_and_inline_emphasis_changes_preserve_link_meaning() {
    for (rule_id, source, expected) in [
        (
            "MD049",
            "[*foo*][id] and [*bar*](/inline)\n\n[id]: /reference\n",
            "[_foo_][id] and [_bar_](/inline)\n\n[id]: /reference\n",
        ),
        (
            "MD050",
            "[**foo**][id] and [**bar**](/inline)\n\n[id]: /reference\n",
            "[__foo__][id] and [__bar__](/inline)\n\n[id]: /reference\n",
        ),
        (
            "MD049",
            "![*alt*][id] and ![*alt*](/image-inline)\n\n[id]: /image-reference\n",
            "![_alt_][id] and ![_alt_](/image-inline)\n\n[id]: /image-reference\n",
        ),
        (
            "MD050",
            "![**alt**][id] and ![**alt**](/image-inline)\n\n[id]: /image-reference\n",
            "![__alt__][id] and ![__alt__](/image-inline)\n\n[id]: /image-reference\n",
        ),
    ] {
        let fixed = MarkdownLinter::fix(source, &style_options(rule_id)).expect("fix should run");
        assert_eq!(
            fixed.content, expected,
            "{rule_id} should fix explicit and inline links"
        );
        assert_eq!(destinations(source), destinations(expected));
        assert_eq!(events(source), events(expected));
    }
}

fn style_options(rule_id: &str) -> LintOptions {
    let raw = if rule_id == "MD049" {
        json!({
            "default": false,
            "MD049": { "enabled": true, "style": "underscore" }
        })
    } else {
        json!({
            "default": false,
            "MD050": { "enabled": true, "style": "underscore" }
        })
    };
    MarkdownLintConfig { raw }.to_lint_options()
}

fn destinations(source: &str) -> Vec<(bool, String)> {
    Parser::new(source)
        .filter_map(|event| match event {
            Event::Start(Tag::Link { dest_url, .. }) => Some((false, dest_url.to_string())),
            Event::Start(Tag::Image { dest_url, .. }) => Some((true, dest_url.to_string())),
            _ => None,
        })
        .collect()
}

fn events(source: &str) -> Vec<String> {
    Parser::new(source)
        .map(|event| format!("{event:?}"))
        .collect()
}

#[test]
fn style_changes_must_not_create_previously_unresolved_reference_bindings() {
    for (rule, source) in [
        ("MD049", "[*foo*]\n\n[_foo_]: /new-link\n"),
        ("MD050", "[**foo**]\n\n[__foo__]: /new-link\n"),
    ] {
        let fixed = MarkdownLinter::fix(source, &style_options(rule)).expect("fix should run");
        assert_eq!(fixed.content, source);
        assert!(destinations(source).is_empty());
        assert!(destinations(&fixed.content).is_empty());
    }
}
