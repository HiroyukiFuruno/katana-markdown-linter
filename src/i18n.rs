use serde::Serialize;
use std::collections::BTreeMap;

pub type MessageParams = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    Ja,
}

impl Locale {
    pub fn resolve(explicit: Option<&str>) -> Result<Self, LocaleError> {
        Self::resolve_with(explicit, |key| std::env::var(key).ok())
    }

    fn resolve_with(
        explicit: Option<&str>,
        get_env: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, LocaleError> {
        if let Some(locale) = explicit {
            return Self::parse(locale).ok_or_else(|| LocaleError {
                locale: locale.to_string(),
            });
        }

        for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Some(value) = get_env(key) {
                if let Some(locale) = Self::parse(&value) {
                    return Ok(locale);
                }
            }
        }

        Ok(Self::En)
    }

    pub fn parse(value: &str) -> Option<Self> {
        let normalized = value
            .split('.')
            .next()
            .unwrap_or(value)
            .replace('_', "-")
            .to_ascii_lowercase();
        let primary = normalized.split('-').next().unwrap_or(normalized.as_str());
        match primary {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleError {
    pub locale: String,
}

impl std::fmt::Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unsupported locale: {} (supported: en, ja)", self.locale)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalizedDiagnostic {
    pub rule_id: String,
    pub rule_name: String,
    pub message: String,
    pub message_id: String,
    pub message_params: MessageParams,
    pub severity: crate::Severity,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub fix: Option<crate::Fix>,
}

impl LocalizedDiagnostic {
    pub fn from_result(result: &crate::LintResult, locale: Locale) -> Self {
        let mut localized = result.clone();
        localized.message = render_message(
            locale,
            result.message_id.as_str(),
            &result.message_params,
            result.message.as_str(),
        );
        Self {
            rule_id: localized.rule_id,
            rule_name: localized.rule_name,
            message: localized.message,
            message_id: localized.message_id,
            message_params: localized.message_params,
            severity: localized.severity,
            line: localized.line,
            column: localized.column,
            end_line: localized.end_line,
            end_column: localized.end_column,
            fix: localized.fix,
        }
    }
}

pub fn diagnostic_message_id(rule_id: &str, message: &str) -> String {
    if rule_id == "MD001" && message.contains("[Expected: h") {
        "rule.MD001.heading_increment".to_string()
    } else {
        "rule.generic".to_string()
    }
}

pub fn diagnostic_message_params(rule_id: &str, rule_name: &str, message: &str) -> MessageParams {
    let mut params = MessageParams::new();
    params.insert("rule_id".to_string(), rule_id.to_string());
    params.insert("rule_name".to_string(), rule_name.to_string());
    params.insert("message".to_string(), message.to_string());

    if rule_id == "MD001" {
        if let Some((expected, actual)) = parse_heading_levels(message) {
            params.insert("expected".to_string(), expected);
            params.insert("actual".to_string(), actual);
        }
    }

    params
}

pub fn render_message(
    locale: Locale,
    message_id: &str,
    params: &MessageParams,
    fallback: &str,
) -> String {
    match locale {
        Locale::En => fallback.to_string(),
        Locale::Ja => match message_id {
            "rule.MD001.heading_increment" => format!(
                "見出しレベルは一度に1段階だけ増やしてください [期待値: {}, 実際: {}]",
                param(params, "expected", "h?"),
                param(params, "actual", "h?")
            ),
            "rule.generic" => japanese_rule_message(param(params, "rule_id", "unknown"), fallback),
            "config.error" => format!("設定エラー: {}", param(params, "message", fallback)),
            "filesystem.error" => {
                format!(
                    "ファイルシステムエラー: {}",
                    param(params, "message", fallback)
                )
            }
            "glob.error" => format!("glob エラー: {}", param(params, "message", fallback)),
            "rule.error" => format!("ルール実行エラー: {}", param(params, "message", fallback)),
            "summary.no_files" => "Markdown ファイルが見つかりません".to_string(),
            "summary.statistics" => format!(
                "files: {}, files_with_issues: {}, issues: {}, fixable: {}, fixed: {}",
                param(params, "files", "0"),
                param(params, "files_with_issues", "0"),
                param(params, "issues", "0"),
                param(params, "fixable", "0"),
                param(params, "fixed", "0")
            ),
            "fix.fixed_count" => format!(
                "{}: {} 件の問題を修正しました",
                param(params, "path", ""),
                param(params, "count", "0")
            ),
            _ => fallback.to_string(),
        },
    }
}

pub fn catalog_keys(locale: Locale) -> &'static [&'static str] {
    match locale {
        Locale::En | Locale::Ja => &CATALOG_KEYS,
    }
}

fn param<'a>(params: &'a MessageParams, key: &str, fallback: &'a str) -> &'a str {
    params.get(key).map(String::as_str).unwrap_or(fallback)
}

fn parse_heading_levels(message: &str) -> Option<(String, String)> {
    let expected = message.split("[Expected: ").nth(1)?.split(',').next()?;
    let actual = message.split("Actual: ").nth(1)?.trim_end_matches(']');
    Some((expected.to_string(), actual.to_string()))
}

fn japanese_rule_message(rule_id: &str, fallback: &str) -> String {
    match rule_id {
        "MD001" => "見出しレベルは一度に1段階だけ増やしてください".to_string(),
        "MD003" => "見出しのスタイルを統一してください".to_string(),
        "MD004" => "箇条書きリストの記号スタイルを統一してください".to_string(),
        "MD005" => "同じレベルのリスト項目のインデントを揃えてください".to_string(),
        "MD007" => "箇条書きリストのインデントを設定に合わせてください".to_string(),
        "MD009" => "行末の余分なスペースを削除してください".to_string(),
        "MD010" => "ハードタブを使用しないでください".to_string(),
        "MD011" => "逆向きのリンク構文を修正してください".to_string(),
        "MD012" => "複数の連続した空行を削減してください".to_string(),
        "MD013" => "行の長さが上限を超えています".to_string(),
        "MD014" => "コマンド例の前に不要なドル記号があります".to_string(),
        "MD018" => "ATX 見出しの # の後にスペースが必要です".to_string(),
        "MD019" => "ATX 見出しの # の後のスペースは1つにしてください".to_string(),
        "MD020" => "閉じた ATX 見出しの内側にスペースが必要です".to_string(),
        "MD021" => "閉じた ATX 見出しの内側のスペースは1つにしてください".to_string(),
        "MD022" => "見出しは空行で囲んでください".to_string(),
        "MD023" => "見出しは行頭から始めてください".to_string(),
        "MD024" => "同じ内容の見出しが複数あります".to_string(),
        "MD025" => "同一文書内のトップレベル見出しは1つにしてください".to_string(),
        "MD026" => "見出し末尾の句読点を削除してください".to_string(),
        "MD027" => "引用記号の後の余分なスペースを削除してください".to_string(),
        "MD028" => "引用ブロック内に不要な空行があります".to_string(),
        "MD029" => "番号付きリストの番号を正しく並べてください".to_string(),
        "MD030" => "リスト記号後のスペース数を揃えてください".to_string(),
        "MD031" => "コードブロックは空行で囲んでください".to_string(),
        "MD032" => "リストは空行で囲んでください".to_string(),
        "MD033" => "インライン HTML を使用しないでください".to_string(),
        "MD034" => "裸の URL は山括弧またはリンクとして記述してください".to_string(),
        "MD035" => "水平線のスタイルを統一してください".to_string(),
        "MD036" => "強調だけの行を見出しとして使用しないでください".to_string(),
        "MD037" => "強調記号の内側にスペースを入れないでください".to_string(),
        "MD038" => "コード記号の内側にスペースを入れないでください".to_string(),
        "MD039" => "リンクテキストの内側に余分なスペースを入れないでください".to_string(),
        "MD040" => "フェンス付きコードブロックには言語を指定してください".to_string(),
        "MD041" => "ファイルの最初の行はトップレベル見出しにしてください".to_string(),
        "MD042" => "空のリンクを使用しないでください".to_string(),
        "MD043" => "必要な見出し構造に合わせてください".to_string(),
        "MD044" => "固有名詞の表記を設定に合わせてください".to_string(),
        "MD045" => "画像には代替テキストを設定してください".to_string(),
        "MD046" => "コードブロックのスタイルを統一してください".to_string(),
        "MD047" => "ファイル末尾には改行を入れてください".to_string(),
        "MD048" => "フェンス付きコードブロックの記号スタイルを統一してください".to_string(),
        "MD049" => "強調のスタイルを統一してください".to_string(),
        "MD050" => "強い強調のスタイルを統一してください".to_string(),
        "MD051" => "リンク先の見出しフラグメントが存在しません".to_string(),
        "MD052" => "参照リンクまたは画像の定義がありません".to_string(),
        "MD053" => "未使用のリンク定義があります".to_string(),
        "MD054" => "リンクと画像のスタイルを設定に合わせてください".to_string(),
        "MD055" => "表の区切り行のスタイルを統一してください".to_string(),
        "MD056" => "表の列数を揃えてください".to_string(),
        "MD058" => "表は空行で囲んでください".to_string(),
        "MD059" => "リンクテキストは説明的にしてください".to_string(),
        "MD060" => "表セルのスペースを揃えてください".to_string(),
        "md-broken-link" => format!("ローカルリンクが壊れています: {fallback}"),
        _ => format!("ルール {rule_id}: {fallback}"),
    }
}

const CATALOG_KEYS: [&str; 9] = [
    "rule.generic",
    "rule.MD001.heading_increment",
    "config.error",
    "filesystem.error",
    "glob.error",
    "rule.error",
    "summary.no_files",
    "summary.statistics",
    "fix.fixed_count",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_parsing_accepts_primary_and_region_values() {
        assert_eq!(Locale::parse("en"), Some(Locale::En));
        assert_eq!(Locale::parse("en-US"), Some(Locale::En));
        assert_eq!(Locale::parse("ja_JP.UTF-8"), Some(Locale::Ja));
        assert_eq!(Locale::parse("fr"), None);
        assert!(Locale::resolve(Some("fr")).is_err());
    }

    #[test]
    fn explicit_locale_overrides_os_locale_and_unsupported_os_falls_back_to_english() {
        assert_eq!(
            Locale::resolve_with(Some("en"), |_| Some("ja_JP.UTF-8".to_string())),
            Ok(Locale::En)
        );
        assert_eq!(
            Locale::resolve_with(None, |_| Some("fr_FR.UTF-8".to_string())),
            Ok(Locale::En)
        );
    }

    #[test]
    fn catalog_keys_match_between_supported_locales() {
        assert_eq!(catalog_keys(Locale::En), catalog_keys(Locale::Ja));
    }

    #[test]
    fn missing_translation_key_falls_back_to_english_message() {
        assert_eq!(
            render_message(
                Locale::Ja,
                "unknown.message",
                &MessageParams::new(),
                "English fallback"
            ),
            "English fallback"
        );
    }

    #[test]
    fn md001_message_params_extract_expected_and_actual_levels() {
        let params = diagnostic_message_params(
            "MD001",
            "heading-increment",
            "Heading levels should only increment by one level at a time [Expected: h2, Actual: h4]",
        );
        assert_eq!(params["expected"], "h2");
        assert_eq!(params["actual"], "h4");
    }
}
