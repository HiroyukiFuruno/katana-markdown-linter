use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// MD051 / link-fragments — Link fragments.
pub struct LinkFragmentsRule;

impl MarkdownRule for LinkFragmentsRule {
    fn id(&self) -> &'static str {
        "MD051"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD051")
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
        let ignore_case = config
            .and_then(|config| config.properties.get("ignore_case"))
            .map(|value| value == "true")
            .unwrap_or(false);
        let ignored_pattern = config
            .and_then(|config| config.properties.get("ignored_pattern"))
            .and_then(|pattern| regex::Regex::new(pattern).ok());
        self.evaluate_context_with_options(ctx, ignore_case, ignored_pattern.as_ref())
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }
}

impl LinkFragmentsRule {
    fn evaluate_context_with_options(
        &self,
        ctx: &DocumentContext<'_>,
        ignore_case: bool,
        ignored_pattern: Option<&regex::Regex>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD051");
        let mut diagnostics = Vec::new();
        let headings = heading_fragments(ctx);
        for link in ctx.inline_links() {
            if !link.kind.is_inline() {
                continue;
            }
            let Some(destination) = link.destination else {
                continue;
            };
            let Some(fragment) = destination.strip_prefix('#') else {
                continue;
            };
            if fragment.is_empty()
                || is_allowed_special_fragment(fragment)
                || ignored_pattern.is_some_and(|pattern| pattern.is_match(fragment))
                || fragment_exists(&headings, fragment, ignore_case)
            {
                continue;
            }

            let replacement = if ignore_case {
                None
            } else {
                headings
                    .iter()
                    .find(|heading| heading.eq_ignore_ascii_case(fragment))
                    .map(|heading| format!("#{heading}"))
            };
            let range = ctx.diagnostic_range(
                link.destination_range
                    .expect("inline link destination should have a source range"),
            );
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: range.start_line,
                    start_column: range.start_column,
                    end_line: range.end_line,
                    end_column: range.end_column,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: replacement.map(|replacement| {
                    crate::rules::markdown::types::DiagnosticFix {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                        replacement,
                    }
                }),
            });
        }
        diagnostics
    }
}

fn heading_fragments(ctx: &DocumentContext<'_>) -> HashSet<String> {
    let mut counts = HashMap::<String, usize>::new();
    let mut fragments = HashSet::new();

    for (idx, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(idx) {
            continue;
        }
        fragments.extend(html_defined_fragments(line.text));
    }

    for heading in ctx.headings() {
        if let Some(anchor) = custom_heading_anchor(heading.text) {
            fragments.insert(anchor.to_string());
        }
        let base = github_heading_slug(heading.text);
        if base.is_empty() {
            continue;
        }
        let count = counts.entry(base.clone()).or_insert(0);
        let fragment = if *count == 0 {
            base.clone()
        } else {
            format!("{base}-{count}")
        };
        *count += 1;
        fragments.insert(fragment);
    }

    fragments
}

fn fragment_exists(fragments: &HashSet<String>, value: &str, ignore_case: bool) -> bool {
    fragments.contains(value)
        || (ignore_case
            && fragments
                .iter()
                .any(|fragment| fragment.eq_ignore_ascii_case(value)))
}

fn github_heading_slug(heading: &str) -> String {
    let heading = heading
        .trim_end_matches('#')
        .trim()
        .split_once("{#")
        .map_or(heading.trim(), |(before, _)| before.trim());
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in heading.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() || ch == '-' || ch == '_' {
            slug.push(ch);
            previous_dash = false;
        } else if ch.is_whitespace() && !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn custom_heading_anchor(heading: &str) -> Option<&str> {
    let start = heading.rfind("{#")?;
    let after_start = &heading[start + 2..];
    let end = after_start.find('}')?;
    let anchor = &after_start[..end];
    if !anchor.is_empty()
        && anchor
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        Some(anchor)
    } else {
        None
    }
}

fn html_defined_fragments(line: &str) -> Vec<String> {
    let mut fragments = Vec::new();
    if let Some(id) = html_attribute_value(line, "id") {
        fragments.push(id);
    }

    let trimmed = line.trim_start();
    let is_anchor = trimmed
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("<a"))
        && trimmed
            .as_bytes()
            .get(2)
            .is_some_and(|byte| byte.is_ascii_whitespace());
    if is_anchor {
        if let Some(name) = html_attribute_value(line, "name") {
            fragments.push(name);
        }
    }

    fragments
}

fn html_attribute_value(line: &str, attribute: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    let needle = format!("{attribute}=");
    let mut search_start = 0;

    while let Some(relative_start) = lower[search_start..].find(&needle) {
        let attr_start = search_start + relative_start;
        if attr_start > 0 {
            let previous = lower.as_bytes()[attr_start - 1];
            if previous.is_ascii_alphanumeric() || previous == b'-' || previous == b'_' {
                search_start = attr_start + needle.len();
                continue;
            }
        }

        let value_start = attr_start + needle.len();
        let quote = line.as_bytes().get(value_start).copied()?;
        if quote != b'"' && quote != b'\'' {
            search_start = value_start;
            continue;
        }
        let content_start = value_start + 1;
        let close = line[content_start..].find(quote as char)?;
        let value = &line[content_start..content_start + close];
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }

    None
}

fn is_allowed_special_fragment(fragment: &str) -> bool {
    fragment == "top" || is_github_line_fragment(fragment)
}

fn is_github_line_fragment(fragment: &str) -> bool {
    let Some(rest) = fragment.strip_prefix('L') else {
        return false;
    };
    !rest.is_empty()
        && rest
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == 'C' || ch == '-' || ch == 'L')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_case_mismatched_heading_fragment() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "# Heading Name\n\n[Link](#Heading-Name)",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("case mismatch should be fixable");
        assert_eq!(fix.replacement, "#heading-name");
    }

    #[test]
    fn reports_missing_heading_fragment_without_fix() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[Link](#missing)");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].fix_info.is_none());
    }

    #[test]
    fn accepts_configured_ignore_case_fragment() {
        let rule = LinkFragmentsRule;
        let config = RuleConfig {
            enabled: true,
            properties: HashMap::from([("ignore_case".to_string(), "true".to_string())]),
        };
        let diagnostics = rule.evaluate_configured(
            Path::new("doc.md"),
            "# Heading Name\n\n[Link](#Heading-Name)",
            Some(&config),
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn accepts_configured_ignored_pattern() {
        let rule = LinkFragmentsRule;
        let config = RuleConfig {
            enabled: true,
            properties: HashMap::from([("ignored_pattern".to_string(), "^figure-".to_string())]),
        };
        let diagnostics =
            rule.evaluate_configured(Path::new("doc.md"), "[Figure](#figure-1)", Some(&config));

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignore_case_missing_fragment_remains_unfixable() {
        let rule = LinkFragmentsRule;
        let config = RuleConfig {
            enabled: true,
            properties: HashMap::from([("ignore_case".to_string(), "true".to_string())]),
        };
        let diagnostics = rule.evaluate_configured(
            Path::new("doc.md"),
            "# Heading Name\n\n[Link](#missing)",
            Some(&config),
        );

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].fix_info.is_none());
    }

    #[test]
    fn accepts_custom_heading_anchor_and_html_fragments() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "# Heading Name {#custom-name}\n<a id=\"bookmark\"></a>\n<a name='legacy'></a>\n\n[Custom](#custom-name)\n[Bookmark](#bookmark)\n[Legacy](#legacy)",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_code_spans_and_unclosed_fragments() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "`[Code](#missing)` and [Unclosed](#missing",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn accepts_duplicate_heading_suffixes_and_line_fragments() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "# Repeat\n# Repeat\n\n[Second](#repeat-1)\n[Line](#L19C5-L21C11)\n[Top](#top)",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn rejects_invalid_custom_heading_anchor() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "# Heading Name {#Invalid Anchor}\n\n[Custom](#Invalid-Anchor)",
        );

        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn html_attribute_parser_skips_prefixed_and_unquoted_attributes() {
        let rule = LinkFragmentsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "<span data-id=\"skip\" id=unquoted id=\"bookmark\"></span>\n\n[Bookmark](#bookmark)",
        );

        assert!(diagnostics.is_empty());
    }
}
