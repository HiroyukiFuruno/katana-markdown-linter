use crate::cli::args::HelpTopic;

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

const RULE_HELP_JA: &str = "\
使い方: kml rule [RULE_ID] [options]

ルール一覧または個別ルールを表示します。

オプション:
  --output json             JSON 形式で出力します。
  --format json             --output json の別名です。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
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

const INIT_CONFIG_HELP_JA: &str = "\
使い方: kml init-config [options]

既定の .markdownlint.json を作成します。

オプション:
  --config <path>           この path に設定ファイルを作成します。
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
";

const LSP_HELP_JA: &str = "\
使い方: kml lsp

language server を stdio で起動します。

オプション:
  --locale <locale>, -l     診断・ヘルプの表示言語を指定します。未指定時は端末のロケールを使います。
  --help, -h                このヘルプを表示します。
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

pub(super) fn japanese_help_text(topic: Option<HelpTopic>) -> &'static str {
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
