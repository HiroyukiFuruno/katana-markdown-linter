mod diagnostic;
mod locale;
mod messages;
mod rule_descriptions;

pub use diagnostic::LocalizedDiagnostic;
pub use locale::{supported_locales, Locale, LocaleError};
pub use messages::{
    catalog_keys, diagnostic_message_id, diagnostic_message_params, render_message, MessageParams,
};
pub use rule_descriptions::has_rule_description_translation;

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
    rule_descriptions::rule_description(resolve_locale_code(language_code), rule_id)
        .unwrap_or(fallback_description)
        .to_string()
}

pub(crate) fn localized_rule_message(rule_id: &str, fallback: &str, locale: Locale) -> String {
    match rule_descriptions::rule_description(locale, rule_id) {
        Some(message) => message.to_string(),
        None if locale == Locale::Ja && rule_id == "md-broken-link" => {
            format!("ローカルリンクが壊れています: {fallback}")
        }
        None if locale == Locale::Ja => format!("ルール {rule_id}: {fallback}"),
        None => fallback.to_string(),
    }
}

#[cfg(test)]
mod tests;
