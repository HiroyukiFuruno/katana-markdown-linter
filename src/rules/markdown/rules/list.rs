use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use crate::types::RuleConfig;
use std::collections::BTreeMap;
use std::path::Path;

/* WHY: Section: List-related markdownlint rule implementations
======================================================= */

/// MD004 / ul-style — Unordered list style. Enforces consistent bullet character.
pub struct UlStyleRule;

impl MarkdownRule for UlStyleRule {
    fn id(&self) -> &'static str {
        "MD004"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD004")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD004");
        let mut diagnostics = Vec::new();
        let mut first_bullet: Option<char> = None;
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();
            if let Some(bullet) = RuleHelpers::get_bullet_char(trimmed) {
                match first_bullet {
                    None => first_bullet = Some(bullet),
                    Some(expected) if bullet != expected => {
                        let bullet_pos = line.find(bullet).unwrap();
                        let fix = crate::rules::markdown::types::DiagnosticFix {
                            start_line: i + 1,
                            start_column: bullet_pos + 1,
                            end_line: i + 1,
                            end_column: bullet_pos + 2,
                            replacement: expected.to_string(),
                        };
                        RuleHelpers::push_diag_with_fix(
                            &mut diagnostics,
                            file_path,
                            i,
                            line,
                            &meta,
                            DiagnosticSeverity::Warning,
                            Some(fix),
                        );
                    }
                    _ => {}
                }
            }
        }
        diagnostics
    }
}

/// MD029 / ol-prefix — Ordered list item prefix.
pub struct OlPrefixRule;

impl MarkdownRule for OlPrefixRule {
    fn id(&self) -> &'static str {
        "MD029"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD029",
            title: "ol-prefix",
            description: "Ordered list item prefix.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md029.md",
            aliases: &["ol-prefix"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop_enum!(
                "style",
                "List style",
                "one_or_ordered",
                &["one", "ordered", "one_or_ordered", "zero"]
            )],
        })
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
        let meta = self.official_meta().expect("always Some for MD029");
        let mut diagnostics = Vec::new();
        let mut expected_numbers = BTreeMap::<usize, OrderedListState>::new();
        let style = OrderedListStyle::from_config(config);
        for (i, line) in ctx.lines().iter().enumerate() {
            let line = line.text;
            let trimmed = line.trim_start();
            let indent = line.len() - trimmed.len();
            if ctx.is_code_line(i) {
                if !trimmed.is_empty() && !is_ordered_list_continuation(indent, &expected_numbers) {
                    expected_numbers.clear();
                }
                continue;
            }
            if let Some(num) = RuleHelpers::get_ordered_number(trimmed) {
                expected_numbers.retain(|level, _| *level <= indent);
                let state = expected_numbers.entry(indent).or_default();
                let expected = state.expected_number(num, style);
                if let Some(expected_number) = expected {
                    let dot_pos = line.find(". ").unwrap();
                    let start_col = line.find(|c: char| c.is_ascii_digit()).unwrap();
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: start_col + 1,
                        end_line: i + 1,
                        end_column: dot_pos + 1,
                        replacement: expected_number.to_string(),
                    };
                    RuleHelpers::push_diag_with_fix(
                        &mut diagnostics,
                        ctx.file_path(),
                        i,
                        line,
                        &meta,
                        DiagnosticSeverity::Warning,
                        Some(fix),
                    );
                }
                state.content_indent = indent + ordered_marker_width(trimmed);
            } else if !trimmed.is_empty()
                && !is_ordered_list_lazy_continuation(indent, trimmed, &expected_numbers)
            {
                expected_numbers.retain(|level, _| *level < indent);
            }
        }
        diagnostics
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderedListState {
    next_number: u32,
    content_indent: usize,
    detected_style: Option<DetectedOrderedListStyle>,
}

impl Default for OrderedListState {
    fn default() -> Self {
        Self {
            next_number: 1,
            content_indent: 0,
            detected_style: None,
        }
    }
}

impl OrderedListState {
    fn expected_number(&mut self, actual: u32, style: OrderedListStyle) -> Option<u32> {
        match style {
            OrderedListStyle::One => self.expected_constant_number(actual, 1),
            OrderedListStyle::Ordered => self.expected_ordered_number(actual),
            OrderedListStyle::OneOrOrdered => self.expected_one_or_ordered(actual),
            OrderedListStyle::Zero => self.expected_constant_number(actual, 0),
        }
    }

    fn expected_constant_number(&mut self, actual: u32, expected: u32) -> Option<u32> {
        self.next_number += 1;
        mismatch_expected(actual, expected)
    }

    fn expected_ordered_number(&mut self, actual: u32) -> Option<u32> {
        let expected = self.next_number;
        self.next_number += 1;
        mismatch_expected(actual, expected)
    }

    fn expected_one_or_ordered(&mut self, actual: u32) -> Option<u32> {
        if self.next_number == 1 && self.detected_style.is_none() {
            if actual == 0 {
                self.detected_style = Some(DetectedOrderedListStyle::Ordered);
                return None;
            }
            self.next_number = 2;
            return mismatch_expected(actual, 1);
        }
        match self.detected_style {
            Some(DetectedOrderedListStyle::One) => self.expected_constant_number(actual, 1),
            Some(DetectedOrderedListStyle::Ordered) => self.expected_ordered_number(actual),
            None if actual == 1 => {
                self.detected_style = Some(DetectedOrderedListStyle::One);
                self.next_number += 1;
                None
            }
            None if actual == self.next_number => {
                self.detected_style = Some(DetectedOrderedListStyle::Ordered);
                self.next_number += 1;
                None
            }
            None => {
                let expected = self.next_number;
                self.detected_style = Some(DetectedOrderedListStyle::Ordered);
                self.next_number += 1;
                Some(expected)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OrderedListStyle {
    One,
    Ordered,
    OneOrOrdered,
    Zero,
}

impl OrderedListStyle {
    fn from_config(config: Option<&RuleConfig>) -> Self {
        match config
            .and_then(|config| config.properties.get("style"))
            .map(String::as_str)
        {
            Some("one") => Self::One,
            Some("ordered") => Self::Ordered,
            Some("zero") => Self::Zero,
            _ => Self::OneOrOrdered,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DetectedOrderedListStyle {
    One,
    Ordered,
}

fn mismatch_expected(actual: u32, expected: u32) -> Option<u32> {
    (actual != expected).then_some(expected)
}

fn ordered_marker_width(trimmed: &str) -> usize {
    trimmed.find(". ").map_or(0, |dot_pos| dot_pos + 2)
}

fn is_ordered_list_continuation(
    indent: usize,
    expected_numbers: &BTreeMap<usize, OrderedListState>,
) -> bool {
    expected_numbers
        .values()
        .any(|state| state.content_indent > 0 && indent >= state.content_indent)
}

fn is_ordered_list_lazy_continuation(
    indent: usize,
    trimmed: &str,
    expected_numbers: &BTreeMap<usize, OrderedListState>,
) -> bool {
    if is_ordered_list_boundary(trimmed) {
        return false;
    }
    expected_numbers
        .iter()
        .any(|(level, state)| state.content_indent > 0 && indent >= *level)
}

fn is_ordered_list_boundary(trimmed: &str) -> bool {
    trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("```")
        || trimmed.starts_with("~~~")
        || is_thematic_break(trimmed)
}

fn is_thematic_break(trimmed: &str) -> bool {
    let mut marker: Option<char> = None;
    let mut count = 0;
    for ch in trimmed.chars() {
        if ch == ' ' || ch == '\t' {
            continue;
        }
        if ch != '-' && ch != '_' && ch != '*' {
            return false;
        }
        match marker {
            Some(existing) if existing != ch => return false,
            None => marker = Some(ch),
            _ => {}
        }
        count += 1;
    }
    count >= 3
}
