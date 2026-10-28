use katana_markdown_linter::rules::markdown::{
    DocumentContext, MarkdownRule, NoBareUrlsRule, NoSpaceInCodeRule,
};
use std::path::Path;

fn context(content: &str) -> DocumentContext<'_> {
    DocumentContext::new(Path::new("spec.md"), content)
}

#[test]
fn gfm_table_escaped_pipes_preserve_cells_and_byte_ranges() {
    let source = "| a\\|b | 日本語 |\n| --- | --- |\n| c\\| | d |\n";
    let ctx = context(source);
    assert_eq!(ctx.tables().len(), 1);
    let table = &ctx.tables()[0];
    assert_eq!(table.rows.len(), 3);
    for row in &table.rows {
        assert_eq!(row.cells.len(), 2);
        for cell in &row.cells {
            assert_eq!(&source[cell.range.start..cell.range.end], cell.text);
        }
    }
    assert_eq!(table.rows[0].cells[0].text, "a\\|b");
}

#[test]
fn gfm_table_escaped_trailing_pipe_is_cell_content() {
    let ctx = context("a | b\\|\n--- | ---\n");
    assert_eq!(ctx.tables().len(), 1);
    let header = &ctx.tables()[0].rows[0];
    assert!(!header.trailing_pipe);
    assert_eq!(header.cells[1].text, "b\\|");
}

#[test]
fn gfm_table_delimiters_accept_one_hyphen_and_only_one_alignment_colon() {
    assert_eq!(context("| A | B |\n| :- | -: |\n").tables().len(), 1);
    assert!(context("| A | B |\n| ::--- | --- |\n").tables().is_empty());
}

#[test]
fn commonmark_tab_separated_atx_heading_trims_closing_sequence() {
    let ctx = context("#\t日本語\t### \t\n");
    assert_eq!(ctx.headings().len(), 1);
    assert_eq!(ctx.headings()[0].text, "日本語");
}

#[test]
fn duplicate_heading_policy_reuses_shared_heading_text() {
    use katana_markdown_linter::rules::markdown::NoDuplicateHeadingRule;
    let source = "# Title\n\n##\tSection ###\n\n## Section\n";
    let diagnostics = NoDuplicateHeadingRule.evaluate(Path::new("spec.md"), source);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].range.start_line, 5);
}

#[test]
fn commonmark_fence_closer_requires_blank_suffix() {
    let ctx = context("```text\n```still code\n# code\n```\n\n# visible\n");
    assert_eq!(ctx.code_blocks().len(), 1);
    assert_eq!(ctx.code_blocks()[0].end_line, 3);
    assert_eq!(ctx.headings().len(), 1);
    assert_eq!(ctx.headings()[0].text, "visible");
}

#[test]
fn commonmark_backtick_fence_info_cannot_contain_backticks() {
    let ctx = context("```bad`info\n# visible\n");
    assert!(ctx.code_blocks().is_empty());
    assert_eq!(ctx.headings().len(), 1);
}

#[test]
fn commonmark_indented_literal_fence_does_not_mask_later_heading() {
    let ctx = context("    ```\n# visible\n");
    assert!(ctx.code_blocks().is_empty());
    assert_eq!(ctx.headings().len(), 1);
}

#[test]
fn commonmark_multiline_code_excludes_links_html_and_bare_urls() {
    let source = "prefix `code\n[x](missing) <b> https://example.com\nend` suffix\n";
    let ctx = context(source);
    assert_eq!(ctx.inline_code_spans().len(), 1);
    assert!(ctx.inline_code_spans()[0].closed);
    assert!(ctx.inline_links().is_empty());
    assert!(ctx.inline_html_elements().is_empty());
    assert!(NoBareUrlsRule
        .evaluate(Path::new("spec.md"), source)
        .is_empty());
    assert!(NoSpaceInCodeRule
        .evaluate(Path::new("spec.md"), source)
        .is_empty());
}

#[test]
fn multiline_code_does_not_cross_blank_or_heading_boundaries() {
    for source in ["`open\n\nclose`\n", "`open\n# heading\nclose`\n"] {
        let ctx = context(source);
        assert!(!ctx.inline_code_spans().iter().any(|span| span.closed));
    }
}

#[test]
fn fence_examples_with_info_on_closer_remain_literal_content() {
    let source = "````markdown\n```\n````markdown\n# literal\n```\n";
    let ctx = context(source);
    assert_eq!(ctx.code_blocks().len(), 1);
    assert_eq!(ctx.code_blocks()[0].end_line, 4);
    assert!(ctx.headings().is_empty());
}

#[test]
fn code_only_whitespace_is_never_destroyed_by_md038() {
    for source in ["`   `\n", "`` \n ``\n", "``\r\ntext\r\n``\r\n"] {
        assert!(NoSpaceInCodeRule
            .evaluate(Path::new("spec.md"), source)
            .is_empty());
    }
}

#[test]
fn tab_and_reversed_link_rules_share_long_multiline_code_protection() {
    use katana_markdown_linter::rules::markdown::{NoHardTabsRule, NoReversedLinksRule};
    use katana_markdown_linter::RuleConfig;
    let source = "before ``literal\ncode\t(Example)[https://example.com]\nend`` after\n";
    let ctx = context(source);
    let config = RuleConfig {
        enabled: true,
        properties: [("code_blocks".to_owned(), "false".to_owned())]
            .into_iter()
            .collect(),
    };
    assert!(NoHardTabsRule
        .evaluate_context(&ctx, Some(&config))
        .is_empty());
    assert!(NoReversedLinksRule.evaluate_context(&ctx, None).is_empty());
}

#[test]
fn escaped_backticks_do_not_protect_real_links() {
    let ctx = context("\\` [link](target)\n");
    assert!(ctx.inline_code_spans().is_empty());
    assert_eq!(ctx.inline_links().len(), 1);
}

#[test]
fn escaped_backtick_run_preserves_the_following_real_code_span() {
    for source in [
        "\\``[ literal ](target)` text `\n",
        "\\``https://example.com` trailing `\n",
    ] {
        let ctx = context(source);
        assert!(ctx.inline_links().is_empty());
        assert!(katana_markdown_linter::rules::markdown::NoSpacesInLinksRule
            .evaluate(Path::new("spec.md"), source)
            .is_empty());
        assert!(NoBareUrlsRule
            .evaluate(Path::new("spec.md"), source)
            .is_empty());
    }
}

#[test]
fn reversed_link_fix_does_not_cross_inline_code_boundaries() {
    use katana_markdown_linter::rules::markdown::NoReversedLinksRule;
    for source in [
        "`(Example)[https://example.com`]\n",
        "(Example)[https://example.com `suffix]`\n",
    ] {
        let diagnostics = NoReversedLinksRule.evaluate(Path::new("spec.md"), source);
        assert!(diagnostics.is_empty(), "{source:?}: {diagnostics:?}");
    }
}

#[test]
fn multiline_space_fix_preserves_crlf_and_unicode_payload() {
    let source = "`` 日本語\r\n本文 ``\r\n";
    let results = NoSpaceInCodeRule.evaluate(Path::new("spec.md"), source);
    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].fix_info.as_ref().unwrap().replacement,
        "``日本語\r\n本文``"
    );
}
