use serde_json::Value;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!("usage: rule_coverage_dashboard <matrix.json> <known-deltas.json> <output.md>");
        std::process::exit(2);
    }

    let matrix: Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let known_deltas: Value = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
    let output = render_dashboard(&matrix, &known_deltas)?;
    fs::write(Path::new(&args[3]), output)?;
    Ok(())
}

fn render_dashboard(matrix: &Value, known_deltas: &Value) -> Result<String, String> {
    let rules = matrix["rules"]
        .as_array()
        .ok_or_else(|| "matrix.rules must be an array".to_string())?;
    let known_delta_rules = known_deltas["deltas"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|delta| delta["rule_id"].as_str())
        .collect::<BTreeSet<_>>();

    let mut out = String::new();
    out.push_str("# Rule Coverage Dashboard\n\n");
    out.push_str("Generated from `tests/fixtures/rule-fixture-matrix.json`.\n\n");
    out.push_str("| Rule | Check | Fix | Config | Edge | Golden | Known Delta |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: | --- | --- |\n");

    for rule in rules {
        let rule_id = rule["rule_id"].as_str().unwrap_or("<unknown>");
        let check = count(rule, "check_pass") + count(rule, "check_fail");
        let fix = count(rule, "fix");
        let config = count(rule, "config_valid") + count(rule, "config_invalid");
        let edge = count(rule, "edge");
        let golden = if ["MD001", "MD022", "MD047"].contains(&rule_id) {
            "baseline"
        } else {
            "pending"
        };
        let known_delta = if known_delta_rules.contains(rule_id) {
            "yes"
        } else {
            "no"
        };
        out.push_str(&format!(
            "| {rule_id} | {check} | {fix} | {config} | {edge} | {golden} | {known_delta} |\n"
        ));
    }

    Ok(out)
}

fn count(rule: &Value, field: &str) -> usize {
    rule[field].as_array().map_or(0, Vec::len)
}
