use katana_markdown_linter::rules::markdown::{
    DocumentContext, EmphasisStyleRule, MarkdownRule, NoReversedLinksRule, StrongStyleRule,
};
use katana_markdown_linter::{LintOptions, MarkdownLinter, RuleCatalogService, RuleConfig};
use std::collections::HashMap;
use std::path::Path;

fn context(source: &str) -> DocumentContext<'_> {
    DocumentContext::new(Path::new("inline-escape.md"), source)
}

fn options_with_styles(styles: &[(&str, &str)]) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in RuleCatalogService::implemented_rules() {
        options.rules.insert(
            rule.id.to_string(),
            RuleConfig {
                enabled: false,
                properties: HashMap::new(),
            },
        );
    }
    for (rule_id, style) in styles {
        options.rules.insert(
            (*rule_id).to_string(),
            RuleConfig {
                enabled: true,
                properties: [("style".to_owned(), (*style).to_owned())]
                    .into_iter()
                    .collect(),
            },
        );
    }
    options
}

#[test]
fn escaped_reversed_link_delimiters_remain_literal() {
    let source = r"\(example\)[https://example.com]";
    let diagnostics = NoReversedLinksRule.evaluate_context(&context(source), None);

    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn odd_and_even_backslash_parity_controls_reversed_link_openers() {
    let escaped = r"\(example)[https://example.com]";
    let active = r"\\(example)[https://example.com]";

    assert!(NoReversedLinksRule
        .evaluate_context(&context(escaped), None)
        .is_empty());
    assert_eq!(
        NoReversedLinksRule
            .evaluate_context(&context(active), None)
            .len(),
        1
    );
}

#[test]
fn escaped_emphasis_marker_does_not_choose_md049_inferred_style() {
    let source = r"\*literal* and _valid_";
    let diagnostics = EmphasisStyleRule.evaluate_context(&context(source), None);

    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn even_backslash_run_leaves_emphasis_delimiter_active() {
    let source = r"\\*literal* and _valid_";
    let diagnostics = EmphasisStyleRule.evaluate_context(&context(source), None);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].fix_info.as_ref().unwrap().replacement,
        "*valid*"
    );
}

#[test]
fn escaped_strong_marker_does_not_choose_md050_inferred_style() {
    let source = r"\**literal** and __valid__";
    let diagnostics = StrongStyleRule.evaluate_context(&context(source), None);

    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn escaped_emphasis_closer_does_not_close_an_incomplete_candidate() {
    let source = r"*literal\* and _valid_";
    assert!(EmphasisStyleRule
        .evaluate_context(&context(source), None)
        .is_empty());
}

#[test]
fn escaped_strong_closer_does_not_close_an_incomplete_candidate() {
    let source = r"**literal\** and __valid__";
    assert!(StrongStyleRule
        .evaluate_context(&context(source), None)
        .is_empty());
}

#[test]
fn md049_keeps_underscore_delimiters_inside_ascii_and_unicode_words_literal() {
    for source in ["*ok* 日本_foo_語", "*ok* a_b_"] {
        let diagnostics = EmphasisStyleRule.evaluate_context(&context(source), None);
        assert!(diagnostics.is_empty(), "{source:?}: {diagnostics:?}");
    }
}

#[test]
fn md050_keeps_double_underscore_delimiters_inside_ascii_and_unicode_words_literal() {
    for source in ["**ok** 日本__foo__語", "**ok** a__b__"] {
        let diagnostics = StrongStyleRule.evaluate_context(&context(source), None);
        assert!(diagnostics.is_empty(), "{source:?}: {diagnostics:?}");
    }
}

#[test]
fn underscore_open_and_close_delimiters_require_their_own_flanking_side() {
    for source in ["*ok* a_foo_!", "*ok* _foo_a"] {
        assert!(EmphasisStyleRule
            .evaluate_context(&context(source), None)
            .is_empty());
    }

    for source in ["**ok** a__foo__!", "**ok** __foo_a"] {
        assert!(StrongStyleRule
            .evaluate_context(&context(source), None)
            .is_empty());
    }
}

#[test]
fn emphasis_and_strong_flanking_accept_commonmark_whitespace_and_punctuation() {
    for source in [
        "*ok* _valid_",
        "*ok*\t_valid_\t",
        "*ok*\u{000C}_valid_\u{000C}",
        "*ok*　_日本語_　",
        "*ok* 。_valid_。",
        "*ok* €_valid_€",
        "*ok* 「_valid_」",
        "*ok* 🧪_valid_🧪",
    ] {
        let diagnostics = EmphasisStyleRule.evaluate_context(&context(source), None);
        assert_eq!(diagnostics.len(), 1, "{source:?}: {diagnostics:?}");
    }

    for source in [
        "**ok** __valid__",
        "**ok**\t__valid__\t",
        "**ok**\u{000C}__valid__\u{000C}",
        "**ok**　__日本語__　",
        "**ok** 。__valid__。",
        "**ok** €__valid__€",
        "**ok** 「__valid__」",
        "**ok** 🧪__valid__🧪",
    ] {
        let diagnostics = StrongStyleRule.evaluate_context(&context(source), None);
        assert_eq!(diagnostics.len(), 1, "{source:?}: {diagnostics:?}");
    }
}

#[test]
fn commonmark_non_space_controls_do_not_enable_underscore_emphasis() {
    for separator in ["\u{000B}", "\u{0085}", "\u{2028}", "\u{2029}"] {
        let source = format!("*ok*{separator}_foo_{separator}");
        let diagnostics = EmphasisStyleRule.evaluate_context(&context(&source), None);
        assert!(diagnostics.is_empty(), "{source:?}: {diagnostics:?}");
    }
}

#[test]
fn md050_preserves_outer_emphasis_when_styling_triple_strong_delimiters() {
    let source = "***hello***\n";
    let fixed = MarkdownLinter::fix(source, &options_with_styles(&[("MD050", "underscore")]))
        .expect("fix should run");

    assert_eq!(fixed.content, "*__hello__*\n");
}

#[test]
fn both_style_rules_keep_nested_emphasis_meaning_for_triple_delimiters() {
    let source = "***hello***\n";
    let fixed = MarkdownLinter::fix(
        source,
        &options_with_styles(&[("MD049", "underscore"), ("MD050", "underscore")]),
    )
    .expect("fix should run");

    assert_eq!(fixed.content, "___hello___\n");
}

#[test]
fn md050_uses_nested_parser_event_for_strong_inside_emphasis() {
    let source = "**outer *inner* text**\n";
    let fixed = MarkdownLinter::fix(source, &options_with_styles(&[("MD050", "underscore")]))
        .expect("fix should run");

    assert_eq!(fixed.content, "__outer *inner* text__\n");
}

#[test]
fn crossing_emphasis_candidate_keeps_diagnostic_without_semantics_changing_fix() {
    let source = "*foo _bar* baz_";
    let options = RuleConfig {
        enabled: true,
        properties: [("style".to_owned(), "underscore".to_owned())]
            .into_iter()
            .collect(),
    };
    let diagnostics = EmphasisStyleRule.evaluate_configured(
        Path::new("inline-escape.md"),
        source,
        Some(&options),
    );

    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].fix_info.is_none());
}

#[test]
fn nested_emphasis_style_change_preserves_parser_structure_and_text() {
    let source = include_str!("fixtures/document-answer-fix/original/original-064.md");
    let options = RuleConfig {
        enabled: true,
        properties: [("style".to_owned(), "underscore".to_owned())]
            .into_iter()
            .collect(),
    };
    let diagnostics = EmphasisStyleRule.evaluate_configured(
        Path::new("inline-escape.md"),
        source,
        Some(&options),
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("equivalent nested style should be fixable")
            .replacement,
        "_nested strong_"
    );
    assert_eq!(
        MarkdownLinter::fix(source, &options_with_styles(&[("MD049", "underscore")]))
            .expect("fix should run")
            .content,
        source.replacen("*nested strong*", "_nested strong_", 1)
    );
}

#[test]
fn emphasis_events_do_not_create_candidates_inside_block_literals() {
    let source = "*valid*\n\n    *indented*\n\n<!--\n*html block*\n-->\n";
    let fixed = MarkdownLinter::fix(source, &options_with_styles(&[("MD049", "underscore")]))
        .expect("fix should run");

    assert_eq!(
        fixed.content,
        "_valid_\n\n    *indented*\n\n<!--\n*html block*\n-->\n"
    );
}

#[test]
fn closed_delimiter_frames_preserve_meaning_across_public_fix_inputs() {
    let cases = [
        (
            "MD049",
            "underscore",
            "before *emphasis* after",
            "before _emphasis_ after",
        ),
        (
            "MD049",
            "underscore",
            "日本語「*強調*」終端",
            "日本語「_強調_」終端",
        ),
        (
            "MD049",
            "asterisk",
            "[label](https://example.invalid) _emphasis_ <span>literal</span>",
            "[label](https://example.invalid) *emphasis* <span>literal</span>",
        ),
        (
            "MD049",
            "underscore",
            r"before \*literal\* and *emphasis* after",
            r"before \*literal\* and _emphasis_ after",
        ),
        (
            "MD050",
            "underscore",
            "before **strong** after",
            "before __strong__ after",
        ),
        (
            "MD050",
            "underscore",
            "前（**強調**）後",
            "前（__強調__）後",
        ),
        (
            "MD050",
            "asterisk",
            "prefix __strong__ suffix",
            "prefix **strong** suffix",
        ),
    ];

    for (rule, style, source, expected) in cases {
        let fixed = MarkdownLinter::fix(source, &options_with_styles(&[(rule, style)]))
            .expect("fix should run");
        assert_eq!(fixed.content, expected, "{rule} {source:?}");
    }
}

#[test]
fn target_underscore_that_cannot_open_in_word_context_has_no_safe_fix() {
    let source = "foo*bar*";
    let options = RuleConfig {
        enabled: true,
        properties: [("style".to_owned(), "underscore".to_owned())]
            .into_iter()
            .collect(),
    };
    let diagnostics = EmphasisStyleRule.evaluate_configured(
        Path::new("inline-escape.md"),
        source,
        Some(&options),
    );

    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].fix_info.is_none());
}

#[test]
fn inline_link_precedence_suppresses_false_emphasis_candidate() {
    let source = "*[foo*](bar)";
    let diagnostics = EmphasisStyleRule.evaluate(Path::new("inline-escape.md"), source);

    assert!(diagnostics.is_empty());
}

#[test]
fn emphasis_style_ranges_are_unicode_safe_and_crlf_independent() {
    let source = "日本語 *valid*\r\n次の行 *valid*\r\n";
    let diagnostics = EmphasisStyleRule.evaluate_context(&context(source), None);

    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn emphasis_scanners_preserve_code_delimiter_protection() {
    let source = "`*literal* and **literal**` _valid_ __valid__\n";
    assert!(EmphasisStyleRule
        .evaluate_context(&context(source), None)
        .is_empty());
    assert!(StrongStyleRule
        .evaluate_context(&context(source), None)
        .is_empty());
}
