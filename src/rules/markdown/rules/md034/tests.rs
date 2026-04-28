use super::*;

#[test]
fn reports_ftp_and_mailto_bare_urls() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "Download from ftp://files.example.com or email mailto:user@example.com.",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("ftp URL should be fixable")
            .replacement,
        "<ftp://files.example.com>"
    );
    assert_eq!(
        diagnostics[1]
            .fix_info
            .as_ref()
            .expect("mailto URL should be fixable")
            .replacement,
        "<mailto:user@example.com>"
    );
}

#[test]
fn ignores_ftp_inside_code_span() {
    let rule = NoBareUrlsRule;
    assert!(rule
        .evaluate(Path::new("doc.md"), "See `ftp://example.com` for details.")
        .is_empty());
}

#[test]
fn ignores_already_bracketed_ftp() {
    let rule = NoBareUrlsRule;
    assert!(rule
        .evaluate(Path::new("doc.md"), "See <ftp://example.com>.")
        .is_empty());
}

#[test]
fn reports_ftps_bare_url() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "Connect to ftps://secure.example.com.");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("ftps URL should be fixable")
            .replacement,
        "<ftps://secure.example.com>"
    );
}

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
