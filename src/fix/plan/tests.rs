use super::*;

#[test]
fn one_reference_conflict_does_not_reparse_every_independent_edit() {
    let source = format!(
        "[***a*** ***b***]\n\n{}\n\n[_**a**_ _**b**_]: /new\n",
        "***word*** ".repeat(126)
    );
    let mut edits = Vec::new();
    let mut replacements = Vec::new();
    for (index, _) in source.match_indices("***") {
        if source[..index].bytes().filter(|byte| *byte == b'*').count() % 6 == 0 {
            let end = index + 3 + source[index + 3..].find("***").unwrap() + 3;
            replacements.push((index, end, format!("_**{}**_", &source[index + 3..end - 3])));
        }
    }
    for (start, end, replacement) in &replacements {
        edits.push(Edit {
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
        });
    }
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
