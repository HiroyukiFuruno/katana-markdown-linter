use katana_markdown_linter::{available_rules, fix, lint, LintOptions, MarkdownLintConfig};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = LintOptions::default();

    let diagnostics = lint("# Title\n\n### Skipped level\n", &options)?;
    println!("string diagnostics: {}", diagnostics.len());

    let fixed = fix("text with trailing spaces  \n", &options)?;
    println!("applied fixes: {}", fixed.applied_fixes);

    let config = MarkdownLintConfig::load(Path::new(".markdownlint.json"))?;
    println!("loaded config: {}", config.raw);

    let markdown_files = lint_markdown_tree(Path::new("."), &options)?;
    println!("checked markdown files: {markdown_files}");

    let rules = available_rules();
    println!("available rules: {}", rules.len());

    Ok(())
}

fn lint_markdown_tree(
    root: &Path,
    options: &LintOptions,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut checked = 0;
    visit_markdown_files(root, &mut |path| {
        let content = fs::read_to_string(path)?;
        let diagnostics = lint(&content, options)?;
        println!("{}: {} diagnostics", path.display(), diagnostics.len());
        checked += 1;
        Ok(())
    })?;
    Ok(checked)
}

fn visit_markdown_files(
    dir: &Path,
    on_file: &mut dyn FnMut(&Path) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path
            .file_name()
            .is_some_and(|name| name == ".git" || name == "target")
        {
            continue;
        }
        if path.is_dir() {
            visit_markdown_files(&path, on_file)?;
        } else if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "md" | "markdown"))
        {
            on_file(&path)?;
        }
    }
    Ok(())
}
