use crate::rules::markdown::MarkdownSemanticSignature;
use crate::{FixSafety, Range};

pub(super) struct Edit<'a> {
    pub(super) byte_start: usize,
    pub(super) byte_end: usize,
    pub(super) replacement: &'a str,
    pub(super) rule_id: &'a str,
    pub(super) range: Range,
    pub(super) safety: FixSafety,
}

pub(super) fn retain_semantics<'a>(
    accepted: &mut Vec<Edit<'a>>,
    skipped: &mut Vec<Edit<'a>>,
    content: &str,
) {
    let mut selected = accepted
        .iter()
        .map(|edit| !super::policy::preserves_meaning(edit.rule_id, edit.safety))
        .collect::<Vec<_>>();
    if selected.iter().all(|selected| *selected) {
        return;
    }

    /* WHY: 意図した構文修復を基準にし、スタイル修正の組合せだけが意味を変えないことを保証する。 */
    let baseline = MarkdownSemanticSignature::new(&render_selected(content, accepted, &selected));
    let candidates = selected
        .iter()
        .enumerate()
        .filter_map(|(index, keep)| (!keep).then_some(index))
        .collect::<Vec<_>>();
    select_preserving(&mut selected, &candidates, &mut |selection| {
        baseline.matches(&render_selected(content, accepted, selection))
    });

    let mut retained = Vec::with_capacity(accepted.len());
    for (edit, keep) in accepted.drain(..).zip(selected) {
        if keep {
            retained.push(edit);
        } else {
            skipped.push(edit);
        }
    }
    *accepted = retained;
}

fn select_preserving(
    selected: &mut [bool],
    candidates: &[usize],
    matches: &mut impl FnMut(&[bool]) -> bool,
) {
    if candidates.is_empty() {
        return;
    }
    for &index in candidates {
        selected[index] = true;
    }
    if matches(selected) {
        return;
    }
    for &index in candidates {
        selected[index] = false;
    }
    if candidates.len() > 1 {
        let (left, right) = candidates.split_at(candidates.len() / 2);
        select_preserving(selected, left, matches);
        select_preserving(selected, right, matches);
    }
}

fn render_selected(content: &str, edits: &[Edit<'_>], selected: &[bool]) -> String {
    render(
        content,
        edits
            .iter()
            .zip(selected)
            .filter_map(|(edit, keep)| keep.then_some(edit)),
    )
}

pub(super) fn render<'a>(content: &str, edits: impl Iterator<Item = &'a Edit<'a>>) -> String {
    let mut fixed = String::with_capacity(content.len());
    let mut cursor = 0;
    for edit in edits {
        fixed.push_str(&content[cursor..edit.byte_start]);
        fixed.push_str(edit.replacement);
        cursor = edit.byte_end;
    }
    fixed.push_str(&content[cursor..]);
    fixed
}

#[cfg(test)]
mod tests;
