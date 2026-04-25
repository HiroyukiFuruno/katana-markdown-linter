use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD013 / line-length — Line length.
pub struct LineLengthRule;

impl MarkdownRule for LineLengthRule {
    fn id(&self) -> &'static str {
        "MD013"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD013")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD013");
        let options = LineLengthOptions::from_config(config);
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            let Some(limit) = options.limit_for(ctx, i) else {
                continue;
            };
            if is_line_too_long(line.text, limit, options.strict, options.stern) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

struct LineLengthOptions {
    line_length: usize,
    code_block_line_length: usize,
    heading_line_length: usize,
    code_blocks: bool,
    headings: bool,
    tables: bool,
    stern: bool,
    strict: bool,
}

impl LineLengthOptions {
    fn from_config(config: Option<&RuleConfig>) -> Self {
        Self {
            line_length: usize_property(config, "line_length", 80),
            code_block_line_length: usize_property(config, "code_block_line_length", 80),
            heading_line_length: usize_property(config, "heading_line_length", 80),
            code_blocks: bool_property(config, "code_blocks", true),
            headings: bool_property(config, "headings", true),
            tables: bool_property(config, "tables", true),
            stern: bool_property(config, "stern", false),
            strict: bool_property(config, "strict", false),
        }
    }

    fn limit_for(&self, ctx: &DocumentContext<'_>, line_index: usize) -> Option<usize> {
        if ctx.is_code_line(line_index) {
            return self.code_blocks.then_some(self.code_block_line_length);
        }
        if ctx
            .headings()
            .iter()
            .any(|heading| heading.line == line_index)
        {
            return self.headings.then_some(self.heading_line_length);
        }
        if ctx
            .tables()
            .iter()
            .any(|table| (table.start_line..=table.end_line).contains(&line_index))
        {
            return self.tables.then_some(self.line_length);
        }
        Some(self.line_length)
    }
}

fn usize_property(config: Option<&RuleConfig>, key: &str, default: usize) -> usize {
    config
        .and_then(|config| config.properties.get(key))
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn bool_property(config: Option<&RuleConfig>, key: &str, default: bool) -> bool {
    config
        .and_then(|config| config.properties.get(key))
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn is_line_too_long(line: &str, limit: usize, strict: bool, stern: bool) -> bool {
    if line.len() <= limit {
        return false;
    }
    if strict {
        return true;
    }
    let tail = &line[limit..];
    if tail.chars().all(|ch| !ch.is_whitespace()) {
        return false;
    }
    stern || tail.chars().any(char::is_whitespace)
}
