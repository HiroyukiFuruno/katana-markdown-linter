use crate::cli::args::HelpTopic;
use crate::i18n::Locale;

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
  --locale <locale>, -l     Use localized output and help text.
  --stdin                   Read Markdown from standard input.
  --fix                     Apply safe fixes during check.
  --ignore-config-errors    Continue after ignoring invalid config entries.
  --unsafe --yes            Allow unsafe fixes for fix or check --fix.
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

const GLOBAL_HELP_JA: &str = "\
katana-markdown-linter (kml)

使い方: kml <command> [options] [paths...]

コマンド:
  check [paths...]          Markdown の lint 診断を表示します。
  fix [paths...]            安全な lint 修正を適用します。
  fmt [paths...]            Markdown のレイアウトを整形します。
  rule [RULE_ID]            ルール一覧または個別ルールを表示します。
  config get|file|schema    解決済み設定、設定ファイル、JSON schema を表示します。
  init-config               既定の .markdownlint.json を作成します。
  lsp                       language server を stdio で起動します。
  version                   kml のバージョンを表示します。
  help                      このヘルプを表示します。

オプション:
  --help, -h                このヘルプを表示します。
  --version, -V, -v         kml のバージョンを表示します。
  --config <path>           指定した markdownlint 設定ファイルを使います。
  --file <path>             入力ファイルを明示的に追加します。
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --stdin                   標準入力から Markdown を読みます。
  --fix                     check 中に安全な修正を適用します。
  --ignore-config-errors    不正な設定項目を無視して続行します。
  --unsafe --yes            fix または check --fix で unsafe fix を許可します。
  --include <glob>          指定 glob に一致する path を含めます。
  --exclude <glob>          指定 glob に一致する path を除外します。
  --no-ignore               .gitignore の絞り込みを無視します。
  --include-ignored         ignore された path も含めます。
  --include-reserved        node_modules などの予約ディレクトリも含めます。
  --force-exclude           明示入力にも exclude glob を適用します。
  --statistics              集計を表示します。
  --quiet                   対応するテキスト出力を抑制します。
  --verbose                 詳細を表示します。
  --diff                    適用した修正の diff を表示します。
";

const CHECK_HELP: &str = "\
Usage: kml check [options] [paths...]

Report Markdown lint diagnostics.

Options:
  --fix                     Apply safe fixes before reporting.
  --unsafe --yes            Allow unsafe fixes when used with --fix.
  --config <path>           Use a specific markdownlint config file.
  --ignore-config-errors    Continue after ignoring invalid config entries.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized output and help text.
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

const CHECK_HELP_JA: &str = "\
使い方: kml check [options] [paths...]

Markdown の lint 診断を表示します。

オプション:
  --fix                     報告前に安全な修正を適用します。
  --unsafe --yes            --fix と併用して unsafe fix を許可します。
  --config <path>           指定した markdownlint 設定ファイルを使います。
  --ignore-config-errors    不正な設定項目を無視して続行します。
  --file <path>             入力ファイルを明示的に追加します。
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --stdin                   標準入力から Markdown を読みます。
  --include <glob>          指定 glob に一致する path を含めます。
  --exclude <glob>          指定 glob に一致する path を除外します。
  --no-ignore               .gitignore の絞り込みを無視します。
  --include-ignored         ignore された path も含めます。
  --include-reserved        node_modules などの予約ディレクトリも含めます。
  --force-exclude           明示入力にも exclude glob を適用します。
  --statistics              集計を表示します。
  --quiet                   テキスト出力を抑制します。
  --verbose                 詳細を表示します。
  --help, -h                このヘルプを表示します。
";

const FIX_HELP: &str = "\
Usage: kml fix [options] [paths...]

Apply safe lint fixes.

Options:
  --unsafe --yes            Allow unsafe fixes.
  --config <path>           Use a specific markdownlint config file.
  --ignore-config-errors    Continue after ignoring invalid config entries.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized output and help text.
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

const FIX_HELP_JA: &str = "\
使い方: kml fix [options] [paths...]

安全な lint 修正を適用します。

オプション:
  --unsafe --yes            unsafe fix を許可します。
  --config <path>           指定した markdownlint 設定ファイルを使います。
  --ignore-config-errors    不正な設定項目を無視して続行します。
  --file <path>             入力ファイルを明示的に追加します。
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --stdin                   標準入力から Markdown を読みます。
  --include <glob>          指定 glob に一致する path を含めます。
  --exclude <glob>          指定 glob に一致する path を除外します。
  --no-ignore               .gitignore の絞り込みを無視します。
  --include-ignored         ignore された path も含めます。
  --include-reserved        node_modules などの予約ディレクトリも含めます。
  --force-exclude           明示入力にも exclude glob を適用します。
  --statistics              集計を表示します。
  --quiet                   テキスト出力を抑制します。
  --verbose                 詳細を表示します。
  --diff                    適用した修正の diff を表示します。
  --help, -h                このヘルプを表示します。
";

const FMT_HELP: &str = "\
Usage: kml fmt [options] [paths...]

Normalize Markdown layout formatting.

Options:
  --config <path>           Use a specific markdownlint config file.
  --file <path>             Add one explicit input file.
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized output and help text.
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

const FMT_HELP_JA: &str = "\
使い方: kml fmt [options] [paths...]

Markdown のレイアウトを整形します。

オプション:
  --config <path>           指定した markdownlint 設定ファイルを使います。
  --file <path>             入力ファイルを明示的に追加します。
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --stdin                   標準入力から Markdown を読みます。
  --include <glob>          指定 glob に一致する path を含めます。
  --exclude <glob>          指定 glob に一致する path を除外します。
  --no-ignore               .gitignore の絞り込みを無視します。
  --include-ignored         ignore された path も含めます。
  --include-reserved        node_modules などの予約ディレクトリも含めます。
  --force-exclude           明示入力にも exclude glob を適用します。
  --statistics              集計を表示します。
  --quiet                   テキスト出力を抑制します。
  --verbose                 詳細を表示します。
  --diff                    適用した整形の diff を表示します。
  --help, -h                このヘルプを表示します。
";

const RULE_HELP: &str = "\
Usage: kml rule [RULE_ID] [options]

List rules or show one rule.

Options:
  --output json             Emit JSON output.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized output and help text.
  --help, -h                Show this help.
";

const RULE_HELP_JA: &str = "\
使い方: kml rule [RULE_ID] [options]

ルール一覧または個別ルールを表示します。

オプション:
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

const CONFIG_HELP: &str = "\
Usage: kml config get|file|schema [options]

Print resolved config, config path, or JSON schema.

Options:
  --config <path>           Use a specific markdownlint config file.
  --output json             Emit JSON output where supported.
  --format json             Alias for --output json.
  --locale <locale>, -l     Use localized output and help text.
  --help, -h                Show this help.
";

const CONFIG_HELP_JA: &str = "\
使い方: kml config get|file|schema [options]

解決済み設定、設定ファイル、JSON schema を表示します。

オプション:
  --config <path>           指定した markdownlint 設定ファイルを使います。
  --output json             対応箇所で JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

const INIT_CONFIG_HELP: &str = "\
Usage: kml init-config [options]

Create a default .markdownlint.json file.

Options:
  --config <path>           Create the config file at this path.
  --locale <locale>, -l     Use localized output and help text.
  --help, -h                Show this help.
";

const INIT_CONFIG_HELP_JA: &str = "\
使い方: kml init-config [options]

既定の .markdownlint.json を作成します。

オプション:
  --config <path>           この path に設定ファイルを作成します。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

const LSP_HELP: &str = "\
Usage: kml lsp

Run the language server over stdio.

Options:
  --locale <locale>, -l     Use localized output and help text.
  --help, -h                Show this help.
";

const LSP_HELP_JA: &str = "\
使い方: kml lsp

language server を stdio で起動します。

オプション:
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

const VERSION_HELP: &str = "\
Usage: kml version

Print the kml version.

Aliases:
  kml --version
  kml -V
  kml -v

Options:
  --locale <locale>, -l     Use localized output and help text.
  --help, -h                Show this help.
";

const VERSION_HELP_JA: &str = "\
使い方: kml version

kml のバージョンを表示します。

別名:
  kml --version
  kml -V
  kml -v

オプション:
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

pub(crate) fn run_help(topic: Option<HelpTopic>, locale: Locale) -> i32 {
    println!("{}", help_text(topic, locale));
    0
}

fn help_text(topic: Option<HelpTopic>, locale: Locale) -> &'static str {
    match locale {
        Locale::Ja => japanese_help_text(topic),
        _ => english_help_text(topic),
    }
}

fn english_help_text(topic: Option<HelpTopic>) -> &'static str {
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

fn japanese_help_text(topic: Option<HelpTopic>) -> &'static str {
    match topic {
        Some(HelpTopic::Check) => CHECK_HELP_JA,
        Some(HelpTopic::Config) => CONFIG_HELP_JA,
        Some(HelpTopic::Fix) => FIX_HELP_JA,
        Some(HelpTopic::Fmt) => FMT_HELP_JA,
        Some(HelpTopic::InitConfig) => INIT_CONFIG_HELP_JA,
        Some(HelpTopic::Lsp) => LSP_HELP_JA,
        Some(HelpTopic::Rule) => RULE_HELP_JA,
        Some(HelpTopic::Version) => VERSION_HELP_JA,
        None => GLOBAL_HELP_JA,
    }
}
