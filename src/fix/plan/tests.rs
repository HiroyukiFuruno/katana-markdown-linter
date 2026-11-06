use super::*;
use crate::{FixSafety, Range};

#[test]
fn one_reference_conflict_does_not_reparse_every_independent_edit() {
    let source = format!(
        "[***a*** ***b***]\n\n{}\n\n[_**a**_ _**b**_]: /new\n",
        "***word*** ".repeat(126)
    );
    let replacements = emphasis_replacements(&source);
    let edits = replacements.iter().map(emphasis_edit).collect::<Vec<_>>();
    assert_eq!(edits.len(), 128);
    let baseline = MarkdownSemanticSignature::new(&source);
    let mut selected = vec![false; edits.len()];
    let mut comparisons = 0;
    select_preserving(
        &mut selected,
        &(0..edits.len()).collect::<Vec<_>>(),
        &mut |selection| {
            comparisons += 1;
            baseline.matches(&render_selected(&source, &edits, selection))
        },
    );
    assert_eq!(selected.iter().filter(|keep| **keep).count(), 127);
    assert!(selected[0]);
    assert!(!selected[1]);
    assert!(comparisons <= 15, "{comparisons} full-document comparisons");
    assert!(baseline.matches(&render_selected(&source, &edits, &selected)));
}

fn emphasis_replacements(source: &str) -> Vec<(usize, usize, String)> {
    let delimiter = "***";
    source
        .match_indices(delimiter)
        .step_by(2)
        .map(|(start, _)| {
            let inner_start = start + delimiter.len();
            let inner_end = inner_start + source[inner_start..].find(delimiter).unwrap();
            let end = inner_end + delimiter.len();
            (
                start,
                end,
                format!("_**{}**_", &source[inner_start..inner_end]),
            )
        })
        .collect()
}

fn emphasis_edit((start, end, replacement): &(usize, usize, String)) -> Edit<'_> {
    Edit {
        byte_start: *start,
        byte_end: *end,
        replacement,
        rule_id: "MD049",
        range: Range {
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        },
        safety: FixSafety::Safe,
    }
}
