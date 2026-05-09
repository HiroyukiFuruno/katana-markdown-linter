use super::super::args::Args;
use super::super::documents::BenchDocuments;
use super::super::measure::measure;
use super::super::report::Case;
use super::super::BenchResult;
use katana_markdown_linter::rules::markdown::DocumentContext;
use std::hint::black_box;
use std::path::Path;

pub(super) fn extend_context_cases(
    cases: &mut Vec<Case>,
    args: &Args,
    documents: &BenchDocuments,
) -> BenchResult<()> {
    cases.push(measure(
        "context_build_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&documents.large));
            Ok(ctx.lines().len())
        },
    )?);
    cases.push(measure(
        "context_heading_index_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&documents.large));
            Ok(ctx.headings().len())
        },
    )?);
    cases.push(measure(
        "context_table_index_large_document",
        args,
        documents.large.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&documents.large));
            Ok(ctx.tables().len())
        },
    )?);
    cases.push(measure(
        "context_inline_token_index_large_document",
        args,
        documents.link_heavy.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&documents.link_heavy));
            Ok(ctx.inline_code_spans().len()
                + ctx.inline_links().len()
                + ctx.reference_definitions().len())
        },
    )?);
    Ok(())
}
