use super::super::common::UnsafeFixPolicy;
use crate::cli::args::Cli;
use crate::cli::reporter::plural;
use crate::config::ConfigLoader;
use crate::i18n::Locale;
use crate::{FixSafety, MarkdownLinter};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

pub(super) fn resolve_unsafe_fix_policy(
    cli: &Cli,
    files: &[PathBuf],
    locale: Locale,
) -> Result<UnsafeFixPolicy, String> {
    if !cli.unsafe_fixes {
        return Ok(UnsafeFixPolicy::default());
    }
    let candidates = collect_unsafe_fix_candidates(files, cli)?;
    if candidates.is_empty() {
        return Ok(UnsafeFixPolicy {
            include_unsafe: false,
            declined: false,
        });
    }
    let mut stderr = io::stderr();
    write_unsafe_fix_summary(&mut stderr, &candidates, locale)?;
    if cli.yes {
        writeln!(stderr, "Unsafe fixes approved by --yes.").map_err(|err| err.to_string())?;
        return Ok(UnsafeFixPolicy {
            include_unsafe: true,
            declined: false,
        });
    }
    if cfg!(test) {
        return Err("unsafe fixes require --yes in non-interactive mode".to_string());
    }
    if !io::stdin().is_terminal() {
        return Err("unsafe fixes require --yes in non-interactive mode".to_string());
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let approved = super::super::config_cmd::prompt_unsafe_confirmation(&mut stdin, &mut stderr)?;
    if approved {
        writeln!(stderr, "Unsafe fixes approved.").map_err(|err| err.to_string())?;
    } else {
        writeln!(stderr, "Unsafe fixes declined; no files changed.")
            .map_err(|err| err.to_string())?;
    }
    Ok(UnsafeFixPolicy {
        include_unsafe: approved,
        declined: !approved,
    })
}

struct UnsafeFixCandidateSummary {
    path: String,
    rule_id: String,
    count: usize,
}

fn collect_unsafe_fix_candidates(
    files: &[PathBuf],
    cli: &Cli,
) -> Result<Vec<UnsafeFixCandidateSummary>, String> {
    let mut summaries = Vec::new();
    for path in files {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let config = match ConfigLoader::load_effective_config(path, cli.config.as_deref()) {
            Ok(config) => config,
            Err(_) => continue,
        };
        if !config.validate_cached_rules().is_empty() {
            continue;
        }
        let options = config.to_lint_options();
        let diagnostics = MarkdownLinter::lint_for_path(path, &content, &options)
            .map_err(|err| err.to_string())?;
        let mut by_rule = std::collections::BTreeMap::<String, usize>::new();
        for diagnostic in diagnostics {
            if diagnostic
                .fix
                .as_ref()
                .is_some_and(|fix| fix.safety == FixSafety::Unsafe)
            {
                *by_rule.entry(diagnostic.rule_id).or_default() += 1;
            }
        }
        summaries.extend(
            by_rule
                .into_iter()
                .map(|(rule_id, count)| UnsafeFixCandidateSummary {
                    path: path.display().to_string(),
                    rule_id,
                    count,
                }),
        );
    }
    Ok(summaries)
}

fn write_unsafe_fix_summary(
    mut writer: impl Write,
    candidates: &[UnsafeFixCandidateSummary],
    _locale: Locale,
) -> Result<(), String> {
    let total = candidates
        .iter()
        .map(|candidate| candidate.count)
        .sum::<usize>();
    writeln!(
        writer,
        "Unsafe fixes requested: {total} candidate{}",
        plural(total)
    )
    .map_err(|err| err.to_string())?;
    for candidate in candidates {
        writeln!(
            writer,
            "- {} {}: {} candidate{}",
            candidate.path,
            candidate.rule_id,
            candidate.count,
            plural(candidate.count)
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}
