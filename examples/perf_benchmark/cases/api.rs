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
    extend_large_cases(cases, args, documents, options)?;
    cases.push(measure(
        "api_lint_many_small_documents",
        args,
        documents.many_small.len(),
        "documents",
        || lint_many_small_documents(documents, options),
    )?);
    extend_heavy_cases(cases, args, documents, options)
}

fn extend_large_cases(
    cases: &mut Vec<Case>,
    args: &Args,
    documents: &BenchDocuments,
    options: &LintOptions,
) -> BenchResult<()> {
    lint_case(
        cases,
        args,
        "api_lint_large_document",
        &documents.large,
        options,
    )?;
    lint_case(
        cases,
        args,
        "api_lint_clean_large_document",
        &documents.clean_large,
        options,
    )?;
    fix_case(
        cases,
        args,
        "api_fix_large_document",
        &documents.large,
        options,
    )?;
    format_case(cases, args, "api_format_large_document", &documents.large)?;
    Ok(())
}

fn extend_heavy_cases(
    cases: &mut Vec<Case>,
    args: &Args,
    documents: &BenchDocuments,
    options: &LintOptions,
) -> BenchResult<()> {
    for (name, document) in [
        ("api_lint_link_heavy_document", &documents.link_heavy),
        (
            "api_lint_inline_code_heavy_document",
            &documents.inline_code_heavy,
        ),
        (
            "api_lint_reference_heavy_document",
            &documents.reference_heavy,
        ),
        ("api_lint_table_heavy_document", &documents.table_heavy),
    ] {
        lint_case(cases, args, name, document, options)?;
    }
    fix_case(
        cases,
        args,
        "api_fix_parser_heavy_document",
        &documents.parser_heavy,
        options,
    )?;
    format_case(
        cases,
        args,
        "api_format_parser_heavy_document",
        &documents.parser_heavy,
    )
}

fn lint_case(
    cases: &mut Vec<Case>,
    args: &Args,
    name: &'static str,
    document: &str,
    options: &LintOptions,
) -> BenchResult<()> {
    cases.push(measure(
        name,
        args,
        document.lines().count(),
        "lines",
        || Ok(MarkdownLinter::lint(black_box(document), black_box(options))?.len()),
    )?);
    Ok(())
}

fn fix_case(
    cases: &mut Vec<Case>,
    args: &Args,
    name: &'static str,
    document: &str,
    options: &LintOptions,
) -> BenchResult<()> {
    cases.push(measure(
        name,
        args,
        document.lines().count(),
        "lines",
        || Ok(MarkdownLinter::fix(black_box(document), black_box(options))?.applied_fixes),
    )?);
    Ok(())
}

fn format_case(
    cases: &mut Vec<Case>,
    args: &Args,
    name: &'static str,
    document: &str,
) -> BenchResult<()> {
    cases.push(measure(
        name,
        args,
        document.lines().count(),
        "lines",
        || format_document(document),
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
