use serde::Serialize;
use std::collections::BTreeMap;

pub type MessageParams = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Locale {
    En,
    Ja,
}

impl Locale {
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ja => "ja",
        }
    }

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

    pub fn resolve_code(value: &str) -> Self {
        Self::resolve_code_or(value, Self::En)
    }

    pub fn resolve_code_or(value: &str, fallback: Self) -> Self {
        Self::parse(value).unwrap_or(fallback)
    }
}

pub fn supported_locales() -> &'static [Locale] {
    &[Locale::En, Locale::Ja]
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

pub fn resolve_locale_code(language_code: &str) -> Locale {
    Locale::resolve_code(language_code)
}

pub fn resolve_locale_code_or(language_code: &str, fallback: Locale) -> Locale {
    Locale::resolve_code_or(language_code, fallback)
}

pub fn localized_rule_description(
    rule_id: &str,
    fallback_description: &str,
    language_code: &str,
) -> String {
    let mut params = MessageParams::new();
    params.insert("rule_id".to_string(), rule_id.to_string());
    params.insert("rule_name".to_string(), String::new());
    params.insert("message".to_string(), fallback_description.to_string());
    render_message(
        resolve_locale_code(language_code),
        "rule.generic",
        &params,
        fallback_description,
    )
}

pub fn has_rule_description_translation(rule_id: &str, locale: Locale) -> bool {
    match locale {
        Locale::En => true,
        Locale::Ja => japanese_rule_description(rule_id).is_some(),
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
            "config.invalid_root" => "設定ルートは JSON object である必要があります".to_string(),
            "config.unknown_rule" => format!(
                "未知の markdownlint rule です: {}",
                param(params, "rule_id", "")
            ),
            "config.unknown_property" => format!(
                "{}.{} は未知の rule property です",
                param(params, "rule_id", ""),
                param(params, "property", "")
            ),
            "config.invalid_type" => format!(
                "{}{} の型が不正です。期待値: {}、実際: {}",
                param(params, "rule_id", ""),
                config_property_suffix(params),
                param(params, "expected", ""),
                param(params, "actual", "")
            ),
            "config.invalid_enum_value" => format!(
                "{}{} の値が不正です。許可値: {}、実際: {}",
                param(params, "rule_id", ""),
                config_property_suffix(params),
                param(params, "allowed", ""),
                param(params, "actual", "")
            ),
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
            "format.formatted_count" => format!(
                "{}: {} 件の整形操作を適用しました",
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

fn config_property_suffix(params: &MessageParams) -> String {
    params
        .get("property")
        .filter(|property| !property.is_empty())
        .map(|property| format!(".{property}"))
        .unwrap_or_default()
}

fn parse_heading_levels(message: &str) -> Option<(String, String)> {
    let expected = message.split("[Expected: ").nth(1)?.split(',').next()?;
    let actual = message.split("Actual: ").nth(1)?.trim_end_matches(']');
    Some((expected.to_string(), actual.to_string()))
}

fn japanese_rule_message(rule_id: &str, fallback: &str) -> String {
    match japanese_rule_description(rule_id) {
        Some(message) => message.to_string(),
        None if rule_id == "md-broken-link" => format!("ローカルリンクが壊れています: {fallback}"),
        None => format!("ルール {rule_id}: {fallback}"),
    }
}

fn japanese_rule_description(rule_id: &str) -> Option<&'static str> {
    match rule_id {
        "MD001" => Some("見出しレベルは一度に1段階だけ増やしてください"),
        "MD003" => Some("見出しのスタイルを統一してください"),
        "MD004" => Some("箇条書きリストの記号スタイルを統一してください"),
        "MD005" => Some("同じレベルのリスト項目のインデントを揃えてください"),
        "MD007" => Some("箇条書きリストのインデントを設定に合わせてください"),
        "MD009" => Some("行末の余分なスペースを削除してください"),
        "MD010" => Some("ハードタブを使用しないでください"),
        "MD011" => Some("逆向きのリンク構文を修正してください"),
        "MD012" => Some("複数の連続した空行を削減してください"),
        "MD013" => Some("行の長さが上限を超えています"),
        "MD014" => Some("コマンド例の前に不要なドル記号があります"),
        "MD018" => Some("ATX 見出しの # の後にスペースが必要です"),
        "MD019" => Some("ATX 見出しの # の後のスペースは1つにしてください"),
        "MD020" => Some("閉じた ATX 見出しの内側にスペースが必要です"),
        "MD021" => Some("閉じた ATX 見出しの内側のスペースは1つにしてください"),
        "MD022" => Some("見出しは空行で囲んでください"),
        "MD023" => Some("見出しは行頭から始めてください"),
        "MD024" => Some("同じ内容の見出しが複数あります"),
        "MD025" => Some("同一文書内のトップレベル見出しは1つにしてください"),
        "MD026" => Some("見出し末尾の句読点を削除してください"),
        "MD027" => Some("引用記号の後の余分なスペースを削除してください"),
        "MD028" => Some("引用ブロック内に不要な空行があります"),
        "MD029" => Some("番号付きリストの番号を正しく並べてください"),
        "MD030" => Some("リスト記号後のスペース数を揃えてください"),
        "MD031" => Some("コードブロックは空行で囲んでください"),
        "MD032" => Some("リストは空行で囲んでください"),
        "MD033" => Some("インライン HTML を使用しないでください"),
        "MD034" => Some("裸の URL は山括弧またはリンクとして記述してください"),
        "MD035" => Some("水平線のスタイルを統一してください"),
        "MD036" => Some("強調だけの行を見出しとして使用しないでください"),
        "MD037" => Some("強調記号の内側にスペースを入れないでください"),
        "MD038" => Some("コード記号の内側にスペースを入れないでください"),
        "MD039" => Some("リンクテキストの内側に余分なスペースを入れないでください"),
        "MD040" => Some("フェンス付きコードブロックには言語を指定してください"),
        "MD041" => Some("ファイルの最初の行はトップレベル見出しにしてください"),
        "MD042" => Some("空のリンクを使用しないでください"),
        "MD043" => Some("必要な見出し構造に合わせてください"),
        "MD044" => Some("固有名詞の表記を設定に合わせてください"),
        "MD045" => Some("画像には代替テキストを設定してください"),
        "MD046" => Some("コードブロックのスタイルを統一してください"),
        "MD047" => Some("ファイル末尾には改行を入れてください"),
        "MD048" => Some("フェンス付きコードブロックの記号スタイルを統一してください"),
        "MD049" => Some("強調のスタイルを統一してください"),
        "MD050" => Some("強い強調のスタイルを統一してください"),
        "MD051" => Some("リンク先の見出しフラグメントが存在しません"),
        "MD052" => Some("参照リンクまたは画像の定義がありません"),
        "MD053" => Some("未使用のリンク定義があります"),
        "MD054" => Some("リンクと画像のスタイルを設定に合わせてください"),
        "MD055" => Some("表の区切り行のスタイルを統一してください"),
        "MD056" => Some("表の列数を揃えてください"),
        "MD058" => Some("表は空行で囲んでください"),
        "MD059" => Some("リンクテキストは説明的にしてください"),
        "MD060" => Some("表セルのスペースを揃えてください"),
        _ => None,
    }
}

const CATALOG_KEYS: [&str; 15] = [
    "rule.generic",
    "rule.MD001.heading_increment",
    "config.error",
    "config.invalid_root",
    "config.unknown_rule",
    "config.unknown_property",
    "config.invalid_type",
    "config.invalid_enum_value",
    "filesystem.error",
    "glob.error",
    "rule.error",
    "summary.no_files",
    "summary.statistics",
    "fix.fixed_count",
    "format.formatted_count",
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
    fn public_locale_resolvers_accept_ui_language_codes() {
        assert_eq!(resolve_locale_code("en"), Locale::En);
        assert_eq!(resolve_locale_code("EN_us.UTF-8"), Locale::En);
        assert_eq!(resolve_locale_code("ja"), Locale::Ja);
        assert_eq!(resolve_locale_code("ja-JP"), Locale::Ja);
        assert_eq!(resolve_locale_code(""), Locale::En);
        assert_eq!(resolve_locale_code("fr"), Locale::En);
        assert_eq!(resolve_locale_code_or("fr", Locale::Ja), Locale::Ja);
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
        for locale in supported_locales() {
            assert_eq!(catalog_keys(Locale::En), catalog_keys(*locale));
        }
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

    #[test]
    fn localized_rule_description_uses_catalog_without_diagnostic_params() {
        assert_eq!(
            localized_rule_description("MD003", "Heading style should be consistent", "ja-JP"),
            "見出しのスタイルを統一してください"
        );
        assert_eq!(
            localized_rule_description("MD003", "Heading style should be consistent", "fr"),
            "Heading style should be consistent"
        );
        assert!(localized_rule_description("MD999", "Custom fallback", "ja")
            .contains("Custom fallback"));
    }

    #[test]
    fn config_error_messages_render_with_structured_params() {
        let mut params = MessageParams::new();
        params.insert("rule_id".to_string(), "MD013".to_string());
        params.insert("property".to_string(), "line_length".to_string());
        params.insert("expected".to_string(), "number".to_string());
        params.insert("actual".to_string(), "string".to_string());

        assert_eq!(
            render_message(
                Locale::Ja,
                "config.invalid_type",
                &params,
                "invalid rule property value"
            ),
            "MD013.line_length の型が不正です。期待値: number、実際: string"
        );
    }

    #[test]
    fn active_rule_description_translation_status_is_explicit() {
        assert!(has_rule_description_translation("MD003", Locale::Ja));
        assert!(!has_rule_description_translation("MD999", Locale::Ja));
    }
}
