use katana_markdown_linter::{
    FormatOptions, I18nRuleDescriptionService, LintOptions, Locale, LocaleService,
    MarkdownFormatter, MarkdownLintConfig, MarkdownLinter, RuleCatalogService,
};
use std::fs;
use std::path::Path;

type DynError = Box<dyn std::error::Error>;
type VisitResult = Result<(), DynError>;
type FileVisitor<'a> = dyn FnMut(&Path) -> VisitResult + 'a;

fn main() -> Result<(), DynError> {
    let options = LintOptions::default();

    let diagnostics = MarkdownLinter::lint("# Title\n\n### Skipped level\n", &options)?;
    println!("string diagnostics: {}", diagnostics.len());
    let unsafe_candidates = MarkdownLinter::lint("**Section**\n\nText\n", &options)?
        .into_iter()
        .filter_map(|diagnostic| diagnostic.fix)
        .filter(|fix| fix.safety == katana_markdown_linter::FixSafety::Unsafe)
        .count();
    println!("unsafe fix candidates: {unsafe_candidates}");

    let fixed = MarkdownLinter::fix("text with trailing spaces  \n", &options)?;
    println!("applied fixes: {}", fixed.applied_fixes);
    let formatted =
        MarkdownFormatter::format_markdown("# Title\r\nText\n\n\n", &FormatOptions::default())?;
    println!("format operations: {}", formatted.applied_operations);

    let config = MarkdownLintConfig::load(Path::new(".markdownlint.json"))?;
    let configured_options = config.to_lint_options();
    println!("loaded config: {}", config.raw);

    let markdown_files = lint_markdown_tree(Path::new("."), &configured_options)?;
    println!("checked markdown files: {markdown_files}");

    let rules = RuleCatalogService::available_rules();
    println!("available rules: {}", rules.len());
    let locale = LocaleService::resolve_code_or("ja-JP", Locale::En);
    println!("resolved locale: {locale:?}");
    let localized_catalog = RuleCatalogService::localized_rule_catalog(locale.code());
    println!(
        "localized active rules: {}",
        localized_catalog.active_rules().count()
    );
    if let Some(rule) = rules.iter().find(|rule| rule.id == "MD003") {
        println!(
            "localized MD003 description: {}",
            rule.localized_description("ja-JP")
        );
    }
    println!(
        "localized fallback: {}",
        I18nRuleDescriptionService::localized_rule_description(
            "MD999",
            "Custom rule description",
            "ja-JP"
        )
    );

    Ok(())
}

fn lint_markdown_tree(root: &Path, options: &LintOptions) -> Result<usize, DynError> {
    let mut checked = 0;
    visit_markdown_files(root, &mut |path| {
        let content = fs::read_to_string(path)?;
        let diagnostics = MarkdownLinter::lint(&content, options)?;
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
