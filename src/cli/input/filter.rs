use super::super::args::Cli;
use super::InputExpandError;
use glob::Pattern;
use std::path::PathBuf;

pub(super) fn filter_paths(
    paths: Vec<PathBuf>,
    cli: &Cli,
    explicit: bool,
) -> Result<Vec<PathBuf>, InputExpandError> {
    let includes = compile_patterns(&cli.include)?;
    let excludes = compile_patterns(&cli.exclude)?;
    Ok(paths
        .into_iter()
        .filter(|path| {
            let text = path.to_string_lossy();
            let included =
                includes.is_empty() || includes.iter().any(|pattern| pattern.matches(&text));
            let excluded = excludes.iter().any(|pattern| pattern.matches(&text));
            included && (!excluded || (explicit && !cli.force_exclude))
        })
        .collect())
}

fn compile_patterns(patterns: &[String]) -> Result<Vec<Pattern>, InputExpandError> {
    patterns
        .iter()
        .map(|pattern| Pattern::new(pattern).map_err(|err| InputExpandError::Glob(err.to_string())))
        .collect()
}
