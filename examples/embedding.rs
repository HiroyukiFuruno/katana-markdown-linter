use katana_markdown_linter::{
    available_rules, fix, lint, localized_rule_description, resolve_locale_code_or, LintOptions,
    Locale, MarkdownLintConfig,
};
use std::fs;
use std::path::Path;

type DynError = Box<dyn std::error::Error>;
type VisitResult = Result<(), DynError>;
type FileVisitor<'a> = dyn FnMut(&Path) -> VisitResult + 'a;

fn main() -> Result<(), DynError> {
    let options = LintOptions::default();

    let diagnostics = lint("# Title\n\n### Skipped level\n", &options)?;
    println!("string diagnostics: {}", diagnostics.len());

    let fixed = fix("text with trailing spaces  \n", &options)?;
    println!("applied fixes: {}", fixed.applied_fixes);

    let config = MarkdownLintConfig::load(Path::new(".markdownlint.json"))?;
    let configured_options = config.to_lint_options();
    println!("loaded config: {}", config.raw);

    let markdown_files = lint_markdown_tree(Path::new("."), &configured_options)?;
    println!("checked markdown files: {markdown_files}");

    let rules = available_rules();
    println!("available rules: {}", rules.len());
    let locale = resolve_locale_code_or("ja-JP", Locale::En);
    println!("resolved locale: {locale:?}");
    if let Some(rule) = rules.iter().find(|rule| rule.id == "MD003") {
        println!(
            "localized MD003 description: {}",
            rule.localized_description("ja-JP")
        );
    }
    println!(
        "localized fallback: {}",
        localized_rule_description("MD999", "Custom rule description", "ja-JP")
    );

    Ok(())
}

fn lint_markdown_tree(root: &Path, options: &LintOptions) -> Result<usize, DynError> {
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

fn visit_markdown_files(dir: &Path, on_file: &mut FileVisitor<'_>) -> Result<(), DynError> {
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
