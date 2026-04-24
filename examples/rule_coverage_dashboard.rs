use serde_json::Value;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 5 {
        eprintln!(
            "usage: rule_coverage_dashboard <matrix.json> <golden-baseline.json> <known-deltas.json> <output.md>"
        );
        std::process::exit(2);
    }

    let matrix: Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let baseline: Value = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
    let known_deltas: Value = serde_json::from_str(&fs::read_to_string(&args[3])?)?;
    let output = render_dashboard(&matrix, &baseline, &known_deltas)?;
    fs::write(Path::new(&args[4]), output)?;
    Ok(())
}

fn render_dashboard(
    matrix: &Value,
    baseline: &Value,
    known_deltas: &Value,
) -> Result<String, String> {
    let rules = matrix["rules"]
        .as_array()
        .ok_or_else(|| "matrix.rules must be an array".to_string())?;
    let golden_rules = baseline["cases"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|case| case["rule_id"].as_str())
        .collect::<BTreeSet<_>>();
    let known_delta_rules = known_deltas["deltas"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|delta| delta["rule_id"].as_str())
        .collect::<BTreeSet<_>>();

    let mut out = String::new();
    out.push_str("# Rule Coverage Dashboard\n\n");
    out.push_str("Generated from `tests/fixtures/rule-fixture-matrix.json`.\n\n");
    out.push_str(
        "| Rule | Check | Safe Fix | Config | Edge | Golden | Known Delta | Manual Required |\n",
    );
    out.push_str("| --- | ---: | ---: | ---: | ---: | --- | --- | --- |\n");

    for rule in rules {
        let rule_id = rule["rule_id"].as_str().unwrap_or("<unknown>");
        let check = count(rule, "check_pass") + count(rule, "check_fail");
        let fix = count(rule, "fix");
        let config = count(rule, "config_valid") + count(rule, "config_invalid");
        let edge = count(rule, "edge");
        let golden = if golden_rules.contains(rule_id) {
            "baseline"
        } else {
            "pending: not locked"
        };
        let known_delta = if known_delta_rules.contains(rule_id) {
            "yes"
        } else {
            "no"
        };
        let manual_required = manual_required(rule);
        out.push_str(&format!(
            "| {rule_id} | {check} | {fix} | {config} | {edge} | {golden} | {known_delta} | {manual_required} |\n"
        ));
    }

    Ok(out)
}

fn manual_required(rule: &Value) -> String {
    let reasons = rule["manual_required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|reason| reason.as_str())
        .map(|reason| reason.replace('|', "\\|"))
        .collect::<Vec<_>>()
        .join("<br>");
    if reasons.is_empty() {
        "-".to_string()
    } else {
        reasons
    }
}

fn count(rule: &Value, field: &str) -> usize {
    rule[field].as_array().map_or(0, Vec::len)
}
