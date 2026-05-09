use super::types::RuleFixtureMatrix;
use std::fs;
use std::path::Path;

pub(super) fn render_fixture_matrix_summary(matrix: &RuleFixtureMatrix) -> String {
    let mut out = String::new();
    out.push_str("# Rule Fixture Matrix\n\n");
    out.push_str(&format!("- total rules: {}\n", matrix.summary.total_rules));
    out.push_str(&format!(
        "- rules with examples: {}\n",
        matrix.summary.rules_with_examples
    ));
    out.push_str(&format!(
        "- rules with fix metadata: {}\n",
        matrix.summary.rules_with_fix_metadata
    ));
    out.push_str(&format!(
        "- rules with parameters: {}\n",
        matrix.summary.rules_with_parameters
    ));
    out.push_str(&format!(
        "- manual required: {}\n\n",
        matrix.summary.manual_required
    ));
    out.push_str(&format!(
        "- missing fixtures: {}\n",
        matrix.summary.missing_fixtures
    ));
    out.push_str(&format!(
        "- stale fixtures: {}\n\n",
        matrix.summary.stale_fixtures
    ));
    out.push_str("| Rule | Aliases | Parameters | Examples | Fixable | Manual Required |\n");
    out.push_str("| --- | --- | ---: | ---: | --- | --- |\n");
    for rule in &matrix.rules {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            rule.rule_id,
            rule.aliases.join(", ").replace('|', "\\|"),
            rule.parameters.len(),
            rule.check_fail.len() + rule.check_pass.len(),
            rule.fixable
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            rule.manual_required.join("<br>").replace('|', "\\|")
        ));
    }
    out
}

pub(super) fn write_fixture_matrix_files(
    matrix: &RuleFixtureMatrix,
    output_dir: &Path,
) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|err| format!("{}: {err}", output_dir.display()))?;
    let json = serde_json::to_string_pretty(matrix).map_err(|err| err.to_string())?;
    fs::write(output_dir.join("rule-fixture-matrix.json"), json)
        .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    fs::write(
        output_dir.join("rule-fixture-matrix.md"),
        render_fixture_matrix_summary(matrix),
    )
    .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    Ok(())
}
