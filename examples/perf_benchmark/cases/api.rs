use super::super::args::Args;
use super::super::documents::BenchDocuments;
use super::super::measure::measure;
use super::super::report::Case;
use super::super::BenchResult;
use katana_markdown_linter::{FormatOptions, LintOptions, MarkdownFormatter, MarkdownLinter};
use std::hint::black_box;

pub(super) fn extend_api_cases(
    cases: &mut Vec<Case>,
    args: &Args,
    documents: &BenchDocuments,
    options: &LintOptions,
) -> BenchResult<()> {
    cases.push(measure(
        "api_lint_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || Ok(MarkdownLinter::lint(black_box(&documents.large), black_box(options))?.len()),
    )?);
    cases.push(measure(
        "api_lint_clean_large_document",
        args,
        documents.clean_large.lines().count(),
        "lines",
        || Ok(MarkdownLinter::lint(black_box(&documents.clean_large), black_box(options))?.len()),
    )?);
    cases.push(measure(
        "api_fix_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || Ok(MarkdownLinter::fix(black_box(&documents.large), black_box(options))?.applied_fixes),
    )?);
    cases.push(measure(
        "api_format_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || format_document(&documents.large),
    )?);
    cases.push(measure(
        "api_lint_many_small_documents",
        args,
        documents.many_small.len(),
        "documents",
        || lint_many_small_documents(documents, options),
    )?);
    cases.push(measure(
        "api_lint_link_heavy_document",
        args,
        documents.link_heavy.lines().count(),
        "lines",
        || Ok(MarkdownLinter::lint(black_box(&documents.link_heavy), black_box(options))?.len()),
    )?);
    cases.push(measure(
        "api_lint_inline_code_heavy_document",
        args,
        documents.inline_code_heavy.lines().count(),
        "lines",
        || {
            Ok(
                MarkdownLinter::lint(black_box(&documents.inline_code_heavy), black_box(options))?
                    .len(),
            )
        },
    )?);
    cases.push(measure(
        "api_lint_reference_heavy_document",
        args,
        documents.reference_heavy.lines().count(),
        "lines",
        || {
            Ok(
                MarkdownLinter::lint(black_box(&documents.reference_heavy), black_box(options))?
                    .len(),
            )
        },
    )?);
    cases.push(measure(
        "api_lint_table_heavy_document",
        args,
        documents.table_heavy.lines().count(),
        "lines",
        || Ok(MarkdownLinter::lint(black_box(&documents.table_heavy), black_box(options))?.len()),
    )?);
    cases.push(measure(
        "api_fix_parser_heavy_document",
        args,
        documents.parser_heavy.lines().count(),
        "lines",
        || {
            Ok(
                MarkdownLinter::fix(black_box(&documents.parser_heavy), black_box(options))?
                    .applied_fixes,
            )
        },
    )?);
    cases.push(measure(
        "api_format_parser_heavy_document",
        args,
        documents.parser_heavy.lines().count(),
        "lines",
        || format_document(&documents.parser_heavy),
    )?);
    Ok(())
}

fn lint_many_small_documents(
    documents: &BenchDocuments,
    options: &LintOptions,
) -> BenchResult<usize> {
    let mut diagnostics = 0;
    for document in &documents.many_small {
        diagnostics += MarkdownLinter::lint(black_box(document), black_box(options))?.len();
    }
    Ok(diagnostics)
}

fn format_document(document: &str) -> BenchResult<usize> {
    Ok(
        MarkdownFormatter::format_markdown(black_box(document), &FormatOptions::default())?
            .applied_operations,
    )
}
