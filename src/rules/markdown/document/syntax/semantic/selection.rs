use super::SemanticIndex;
use crate::rules::markdown::document::SourceRange;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScopePreference {
    Inline(usize),
    Block(usize),
    FullDocument,
}

impl SemanticIndex {
    pub(super) fn scope_preference(
        &self,
        range: SourceRange,
        semantic_scope: usize,
        block_scope: Option<usize>,
    ) -> ScopePreference {
        if self.block_has_global_reference_syntax(range, block_scope) {
            return ScopePreference::FullDocument;
        }
        if self.inline_scopes.get(semantic_scope).is_some() {
            return ScopePreference::Inline(semantic_scope);
        }
        block_scope
            .filter(|index| self.block_scopes.get(*index).is_some())
            .map_or(ScopePreference::FullDocument, ScopePreference::Block)
    }

    fn block_has_global_reference_syntax(
        &self,
        candidate: SourceRange,
        block_scope: Option<usize>,
    ) -> bool {
        if !(self.has_reference_definitions || self.has_footnote_syntax) {
            return false;
        }
        let Some(scope) = block_scope.and_then(|index| self.block_scopes.get(index)) else {
            return true;
        };
        candidate.start < scope.range.start
            || candidate.end > scope.range.end
            || scope.has_link_brackets
    }
}
