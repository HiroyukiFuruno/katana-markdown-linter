mod diagnostic;
mod locale;
mod messages;
mod rule_descriptions;

pub use diagnostic::LocalizedDiagnostic;
pub use locale::{Locale, LocaleError, LocaleService};
pub use messages::{MessageCatalog, MessageParams};

pub struct I18nRuleDescriptionService;

impl I18nRuleDescriptionService {
    pub fn localized_rule_description(
        rule_id: &str,
        fallback_description: &str,
        language_code: &str,
    ) -> String {
        rule_descriptions::rule_description(LocaleService::resolve_code(language_code), rule_id)
            .unwrap_or(fallback_description)
            .to_string()
    }

    pub fn has_rule_description_translation(rule_id: &str, locale: Locale) -> bool {
        locale == Locale::En || rule_descriptions::rule_description(locale, rule_id).is_some()
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
}

#[cfg(test)]
mod tests;
