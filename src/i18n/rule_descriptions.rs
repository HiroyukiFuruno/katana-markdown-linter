mod de;
mod es;
mod fr;
mod it;
mod ja;
mod ko;
mod pt;
mod zh_cn;
mod zh_tw;

use super::Locale;

pub(crate) fn rule_description(locale: Locale, rule_id: &str) -> Option<&'static str> {
    match locale {
        Locale::En => None,
        Locale::Ja => ja::get(rule_id),
        Locale::ZhCn => zh_cn::get(rule_id),
        Locale::ZhTw => zh_tw::get(rule_id),
        Locale::Ko => ko::get(rule_id),
        Locale::Pt => pt::get(rule_id),
        Locale::Fr => fr::get(rule_id),
        Locale::De => de::get(rule_id),
        Locale::Es => es::get(rule_id),
        Locale::It => it::get(rule_id),
    }
}

pub fn has_rule_description_translation(rule_id: &str, locale: Locale) -> bool {
    locale == Locale::En || rule_description(locale, rule_id).is_some()
}
