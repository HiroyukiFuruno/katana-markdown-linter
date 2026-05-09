use super::*;
use crate::rules::markdown::MarkdownRule;
use crate::RuleConfig;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn md027_fixes_extra_spaces_after_blockquote_marker() {
    let rule = NoMultipleSpaceBlockquoteRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), ">  quote");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("fix exists")
            .replacement,
        " "
    );
}

#[test]
fn md027_can_ignore_list_items_when_configured() {
    let rule = NoMultipleSpaceBlockquoteRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("list_items".to_string(), "false".to_string())]),
    };
    let diagnostics =
        rule.evaluate_configured(Path::new("doc.md"), ">  - nested item", Some(&config));

    assert!(diagnostics.is_empty());
}
