use super::LineOffsetIndex;
use crate::{FixDetail, FixSafety, LintResult, Range};
use std::{cmp::Ordering, collections::HashSet};

pub(super) struct Edit<'a> {
    pub(super) byte_start: usize,
    pub(super) byte_end: usize,
    pub(super) replacement: &'a str,
    pub(super) rule_id: &'a str,
    pub(super) range: Range,
    pub(super) safety: FixSafety,
}

impl<'a> Edit<'a> {
    pub(super) fn from_diagnostic(
        result: &'a LintResult,
        line_index: &LineOffsetIndex<'_>,
        include_unsafe: bool,
    ) -> Option<Self> {
        let fix = result.fix.as_ref()?;
        if fix.safety == FixSafety::Unsafe && !include_unsafe {
            return None;
        }
        let offsets = line_index.offsets_for_range(&fix.range)?;
        Some(Self {
            byte_start: offsets.start,
            byte_end: offsets.end,
            replacement: &fix.replacement,
            rule_id: &result.rule_id,
            range: fix.range.clone(),
            safety: fix.safety,
        })
    }

    fn priority(&self, other: &Self) -> Ordering {
        other
            .byte_start
            .cmp(&self.byte_start)
            .then_with(|| other.byte_end.cmp(&self.byte_end))
            .then_with(|| self.rule_id.cmp(other.rule_id))
            .then_with(|| self.replacement.cmp(other.replacement))
    }

    fn detail(&self, applied: bool) -> FixDetail {
        FixDetail {
            rule_id: self.rule_id.to_string(),
            range: self.range.clone(),
            applied,
        }
    }
}

#[derive(Default)]
pub(super) struct EditSelection<'a> {
    pub(super) accepted: Vec<Edit<'a>>,
    pub(super) skipped: Vec<Edit<'a>>,
    ineffective: Vec<Edit<'a>>,
}

impl<'a> EditSelection<'a> {
    pub(super) fn non_overlapping(mut edits: Vec<Edit<'a>>, content: &str) -> Self {
        edits.sort_by(Edit::priority);
        let mut selection = Self::default();
        let mut insertion_offsets = HashSet::new();
        let mut previous_start = content.len();
        for edit in edits {
            if content.get(edit.byte_start..edit.byte_end) == Some(edit.replacement) {
                selection.ineffective.push(edit);
                continue;
            }
            let duplicate_insertion =
                edit.byte_start == edit.byte_end && !insertion_offsets.insert(edit.byte_start);
            if duplicate_insertion || edit.byte_end > previous_start {
                selection.skipped.push(edit);
            } else {
                previous_start = edit.byte_start;
                selection.accepted.push(edit);
            }
        }
        selection.accepted.reverse();
        selection
    }

    pub(super) fn details(&self) -> Vec<FixDetail> {
        self.accepted
            .iter()
            .map(|edit| edit.detail(true))
            .chain(
                self.skipped
                    .iter()
                    .chain(&self.ineffective)
                    .map(|edit| edit.detail(false)),
            )
            .collect()
    }
}
