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
    context_case(
        cases,
        args,
        "context_build_large_document",
        &documents.large,
        |ctx| ctx.lines().len(),
    )?;
    context_case(
        cases,
        args,
        "context_heading_index_large_document",
        &documents.large,
        |ctx| ctx.headings().len(),
    )?;
    context_case(
        cases,
        args,
        "context_table_index_large_document",
        &documents.large,
        |ctx| ctx.tables().len(),
    )?;
    inline_context_case(cases, args, &documents.link_heavy)
}

fn inline_context_case(cases: &mut Vec<Case>, args: &Args, document: &str) -> BenchResult<()> {
    context_case(
        cases,
        args,
        "context_inline_token_index_large_document",
        document,
        |ctx| {
            ctx.inline_code_spans().len()
                + ctx.inline_links().len()
                + ctx.reference_definitions().len()
        },
    )
}

fn context_case(
    cases: &mut Vec<Case>,
    args: &Args,
    name: &'static str,
    document: &str,
    count: impl Fn(&DocumentContext) -> usize,
) -> BenchResult<()> {
    cases.push(measure(
        name,
        args,
        document.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(document));
            Ok(count(&ctx))
        },
    )?);
    Ok(())
}
