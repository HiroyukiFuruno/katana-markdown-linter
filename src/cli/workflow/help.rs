use crate::cli::args::HelpTopic;

const GLOBAL_HELP: &str = "\
katana-markdown-linter (kml)

Usage: kml <command> [options] [paths...]

Commands:
  check [paths...]          Report Markdown lint diagnostics.
  fix [paths...]            Apply safe lint fixes.
  fmt [paths...]            Normalize Markdown layout formatting.
  rule [RULE_ID]            List rules or show one rule.
  config get|file|schema    Print resolved config, config path, or JSON schema.
  init-config               Create a default .markdownlint.json file.
  lsp                       Run the language server over stdio.
  version                   Print the kml version.
  help                      Show this help.

Options:
  --help, -h                Show this help.
  --version, -V, -v         Print the kml version.
  --config <path>           Use a specific markdownlint config file.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized rule text.
  --stdin                   Read Markdown from standard input.
  --fix                     Apply safe fixes during check.
  --unsafe --yes            Allow unsafe fixes for the fix command.
  --include <glob>          Include paths matching a glob.
  --exclude <glob>          Exclude paths matching a glob.
  --no-ignore               Ignore .gitignore filtering.
  --include-ignored         Include ignored paths.
  --include-reserved        Include reserved directories such as node_modules.
  --force-exclude           Apply exclude globs to explicit inputs.
  --statistics              Print summary statistics.
  --quiet                   Suppress text output where supported.
  --verbose                 Print extra details where supported.
  --diff                    Print diffs for applied fixes.
";

const CHECK_HELP: &str = "\
Usage: kml check [options] [paths...]

Report Markdown lint diagnostics.

Options:
  --fix                     Apply safe fixes before reporting.
  --config <path>           Use a specific markdownlint config file.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized rule text.
  --stdin                   Read Markdown from standard input.
  --include <glob>          Include paths matching a glob.
  --exclude <glob>          Exclude paths matching a glob.
  --no-ignore               Ignore .gitignore filtering.
  --include-ignored         Include ignored paths.
  --include-reserved        Include reserved directories such as node_modules.
  --force-exclude           Apply exclude globs to explicit inputs.
  --statistics              Print summary statistics.
  --quiet                   Suppress text output.
  --verbose                 Print extra details.
  --help, -h                Show this help.
";

const FIX_HELP: &str = "\
Usage: kml fix [options] [paths...]

Apply safe lint fixes.

Options:
  --unsafe --yes            Allow unsafe fixes.
  --config <path>           Use a specific markdownlint config file.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized rule text.
  --stdin                   Read Markdown from standard input.
  --include <glob>          Include paths matching a glob.
  --exclude <glob>          Exclude paths matching a glob.
  --no-ignore               Ignore .gitignore filtering.
  --include-ignored         Include ignored paths.
  --include-reserved        Include reserved directories such as node_modules.
  --force-exclude           Apply exclude globs to explicit inputs.
  --statistics              Print summary statistics.
  --quiet                   Suppress text output.
  --verbose                 Print extra details.
  --diff                    Print diffs for applied fixes.
  --help, -h                Show this help.
";

const FMT_HELP: &str = "\
Usage: kml fmt [options] [paths...]

Normalize Markdown layout formatting.

Options:
  --config <path>           Use a specific markdownlint config file.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized rule text.
  --stdin                   Read Markdown from standard input.
  --include <glob>          Include paths matching a glob.
  --exclude <glob>          Exclude paths matching a glob.
  --no-ignore               Ignore .gitignore filtering.
  --include-ignored         Include ignored paths.
  --include-reserved        Include reserved directories such as node_modules.
  --force-exclude           Apply exclude globs to explicit inputs.
  --statistics              Print summary statistics.
  --quiet                   Suppress text output.
  --verbose                 Print extra details.
  --diff                    Print diffs for applied formatting.
  --help, -h                Show this help.
";

const RULE_HELP: &str = "\
Usage: kml rule [RULE_ID] [options]

List rules or show one rule.

Options:
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized rule text.
  --help, -h                Show this help.
";

const CONFIG_HELP: &str = "\
Usage: kml config get|file|schema [options]

Print resolved config, config path, or JSON schema.

Options:
  --config <path>           Use a specific markdownlint config file.
  --output json             Emit JSON output where supported.
  --format json             Alias for --output json.
  --help, -h                Show this help.
";

const INIT_CONFIG_HELP: &str = "\
Usage: kml init-config [options]

Create a default .markdownlint.json file.

Options:
  --config <path>           Create the config file at this path.
  --help, -h                Show this help.
";

const LSP_HELP: &str = "\
Usage: kml lsp

Run the language server over stdio.

Options:
  --help, -h                Show this help.
";

const VERSION_HELP: &str = "\
Usage: kml version

Print the kml version.

Aliases:
  kml --version
  kml -V
  kml -v

Options:
  --help, -h                Show this help.
";

pub(crate) fn run_help(topic: Option<HelpTopic>) -> i32 {
    println!("{}", help_text(topic));
    0
}

fn help_text(topic: Option<HelpTopic>) -> &'static str {
    match topic {
        Some(HelpTopic::Check) => CHECK_HELP,
        Some(HelpTopic::Config) => CONFIG_HELP,
        Some(HelpTopic::Fix) => FIX_HELP,
        Some(HelpTopic::Fmt) => FMT_HELP,
        Some(HelpTopic::InitConfig) => INIT_CONFIG_HELP,
        Some(HelpTopic::Lsp) => LSP_HELP,
        Some(HelpTopic::Rule) => RULE_HELP,
        Some(HelpTopic::Version) => VERSION_HELP,
        None => GLOBAL_HELP,
    }
}
