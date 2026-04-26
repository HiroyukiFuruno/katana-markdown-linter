use super::*;

#[test]
fn reports_http_and_https_urls_with_safe_fixes() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "See http://example.com and https://example.org.",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("http URL should be fixable")
            .replacement,
        "<http://example.com>"
    );
    assert_eq!(
        diagnostics[1]
            .fix_info
            .as_ref()
            .expect("https URL should be fixable")
            .replacement,
        "<https://example.org>"
    );
}

#[test]
fn ignores_urls_inside_shared_contexts() {
    let rule = NoBareUrlsRule;
    let content = concat!(
        "[inline](https://example.com)\n",
        "(reversed)[https://example.org]\n",
        "`https://example.invalid`\n",
        "[ref]: https://example.net\n",
        "<a href=\"https://example.jp\">link</a>\n",
    );

    assert!(rule.evaluate(Path::new("doc.md"), content).is_empty());
}

#[test]
fn trims_unmatched_closing_delimiters_from_fix_range() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "See (https://example.com].");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("URL should be fixable")
            .replacement,
        "<https://example.com>"
    );
}
