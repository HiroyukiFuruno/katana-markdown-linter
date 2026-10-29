use super::events::{normalized_signature, SemanticEvent};
use super::selection::ScopePreference;
use super::{SemanticIndex, SemanticScope};
use crate::rules::markdown::document::syntax::COMMONMARK_OPTIONS;
use crate::rules::markdown::document::SourceRange;
use pulldown_cmark::Parser;

#[cfg(test)]
mod tests;

impl SemanticIndex {
    pub(in crate::rules::markdown::document) fn replacement_preserves_inline_semantics(
        &self,
        content: &str,
        range: SourceRange,
        replacement: &str,
        semantic_scope: usize,
        block_scope: Option<usize>,
    ) -> bool {
        match self.scope_preference(range, semantic_scope, block_scope) {
            ScopePreference::FullDocument => {
                self.full_source_matches_replacement(content, range, replacement)
            }
            ScopePreference::Inline(index) => {
                let scope = &self.inline_scopes[index];
                if let Some(equal) = self.compare_inline_scope(content, range, replacement, scope) {
                    return equal;
                }
                self.compare_block_or_full(content, range, replacement, block_scope)
            }
            ScopePreference::Block(index) => {
                let scope = &self.block_scopes[index];
                self.compare_block_scope(content, range, replacement, scope)
                    .unwrap_or_else(|| {
                        self.full_source_matches_replacement(content, range, replacement)
                    })
            }
        }
    }

    fn compare_block_or_full(
        &self,
        content: &str,
        range: SourceRange,
        replacement: &str,
        block_scope: Option<usize>,
    ) -> bool {
        if let Some(scope) = block_scope.and_then(|index| self.block_scopes.get(index)) {
            if let Some(equal) = self.compare_block_scope(content, range, replacement, scope) {
                return equal;
            }
        }
        self.full_source_matches_replacement(content, range, replacement)
    }

    fn compare_inline_scope(
        &self,
        original: &str,
        candidate: SourceRange,
        replacement: &str,
        scope: &SemanticScope,
    ) -> Option<bool> {
        let original_scope = original.get(scope.range.start..scope.range.end)?;
        let changed_scope = replace_source_range(
            original_scope,
            SourceRange {
                start: candidate.start.checked_sub(scope.range.start)?,
                end: candidate.end.checked_sub(scope.range.start)?,
            },
            replacement,
        )?;
        if !crate::rules::markdown::inline::isolated_outer_delimiter_scope(
            original,
            scope.range,
            original_scope,
            &changed_scope,
        ) {
            return None;
        }
        let events = self.events.get_or_init(|| parse_events(original));
        let expected = normalized_signature(events.get(scope.event_range.clone())?);
        let actual_original = inline_fragment_signature(original_scope)?;
        if expected != actual_original {
            return None;
        }
        Some(actual_original == inline_fragment_signature(&changed_scope)?)
    }

    fn compare_block_scope(
        &self,
        original: &str,
        candidate: SourceRange,
        replacement: &str,
        scope: &SemanticScope,
    ) -> Option<bool> {
        let original_scope = original.get(scope.range.start..scope.range.end)?;
        let changed_scope = replace_source_range(
            original_scope,
            SourceRange {
                start: candidate.start.checked_sub(scope.range.start)?,
                end: candidate.end.checked_sub(scope.range.start)?,
            },
            replacement,
        )?;
        let events = self.events.get_or_init(|| parse_events(original));
        let expected = normalized_signature(events.get(scope.event_range.clone())?);
        let actual_original = parse_signature(original_scope);
        if expected != actual_original {
            return None;
        }
        Some(actual_original == parse_signature(&changed_scope))
    }

    fn full_source_matches_replacement(
        &self,
        content: &str,
        range: SourceRange,
        replacement: &str,
    ) -> bool {
        replace_source_range(content, range, replacement)
            .is_some_and(|changed| self.full_source_matches(content, &changed))
    }

    fn full_source_matches(&self, original: &str, changed: &str) -> bool {
        let events = self.events.get_or_init(|| parse_events(original));
        normalized_signature(events) == parse_signature(changed)
    }
}

fn replace_source_range(source: &str, range: SourceRange, replacement: &str) -> Option<String> {
    let prefix = source.get(..range.start)?;
    let suffix = source.get(range.end..)?;
    Some(format!("{prefix}{replacement}{suffix}"))
}

pub(super) fn parse_signature(source: &str) -> Vec<SemanticEvent> {
    normalized_signature(&parse_events(source))
}

fn inline_fragment_signature(source: &str) -> Option<Vec<SemanticEvent>> {
    let mut events = parse_events(source);
    if !matches!(events.first(), Some(SemanticEvent::ParagraphStart))
        || !matches!(events.last(), Some(SemanticEvent::ParagraphEnd))
    {
        return None;
    }
    events.remove(0);
    events.pop();
    Some(normalized_signature(&events))
}

fn parse_events(source: &str) -> Vec<SemanticEvent> {
    Parser::new_ext(source, COMMONMARK_OPTIONS)
        .map(|event| SemanticEvent::from_parser(&event))
        .collect()
}
