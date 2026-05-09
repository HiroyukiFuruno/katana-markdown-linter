use std::collections::BTreeMap;

use super::Locale;

pub type MessageParams = BTreeMap<String, String>;

pub struct MessageCatalog;

impl MessageCatalog {
    pub fn diagnostic_message_id(rule_id: &str, message: &str) -> String {
        if rule_id == "MD001" && message.contains("[Expected: h") {
            "rule.MD001.heading_increment".to_string()
        } else {
            "rule.generic".to_string()
        }
    }

    pub fn diagnostic_message_params(
        rule_id: &str,
        rule_name: &str,
        message: &str,
    ) -> MessageParams {
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
        if message_id == "rule.generic" {
            return super::I18nRuleDescriptionService::localized_rule_message(
                param(params, "rule_id", "unknown"),
                fallback,
                locale,
            );
        }

        match locale {
            Locale::Ja => render_japanese_message(message_id, params, fallback),
            _ => fallback.to_string(),
        }
    }

    pub fn catalog_keys(_locale: Locale) -> &'static [&'static str] {
        &CATALOG_KEYS
    }
}

fn render_japanese_message(message_id: &str, params: &MessageParams, fallback: &str) -> String {
    match message_id {
        "rule.MD001.heading_increment" => format!(
            "見出しレベルは一度に1段階だけ増やしてください [期待値: {}, 実際: {}]",
            param(params, "expected", "h?"),
            param(params, "actual", "h?")
        ),
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
        "config.fix_or_ignore_advice" => {
            "markdownlint 設定を修正するか、無効な設定項目を無視して続行する場合は --ignore-config-errors を付けてください".to_string()
        }
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

const CATALOG_KEYS: [&str; 16] = [
    "rule.generic",
    "rule.MD001.heading_increment",
    "config.error",
    "config.invalid_root",
    "config.unknown_rule",
    "config.unknown_property",
    "config.invalid_type",
    "config.invalid_enum_value",
    "config.fix_or_ignore_advice",
    "filesystem.error",
    "glob.error",
    "rule.error",
    "summary.no_files",
    "summary.statistics",
    "fix.fixed_count",
    "format.formatted_count",
];
