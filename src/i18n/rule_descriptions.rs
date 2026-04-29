use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

use super::Locale;

pub(crate) fn rule_description(locale: Locale, rule_id: &str) -> Option<&'static str> {
    catalog(locale)?
        .rule_descriptions
        .get(rule_id)
        .map(String::as_str)
}

pub fn has_rule_description_translation(rule_id: &str, locale: Locale) -> bool {
    locale == Locale::En || rule_description(locale, rule_id).is_some()
}

#[derive(Debug, Deserialize)]
struct LocaleCatalog {
    rule_descriptions: BTreeMap<String, String>,
}

static JA_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static ZH_CN_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static ZH_TW_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static KO_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static PT_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static FR_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static DE_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static ES_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();
static IT_CATALOG: OnceLock<LocaleCatalog> = OnceLock::new();

fn catalog(locale: Locale) -> Option<&'static LocaleCatalog> {
    match locale {
        Locale::En => None,
        Locale::Ja => Some(load_catalog(
            &JA_CATALOG,
            include_str!("locales/ja.json"),
            "ja",
        )),
        Locale::ZhCn => Some(load_catalog(
            &ZH_CN_CATALOG,
            include_str!("locales/zh-CN.json"),
            "zh-CN",
        )),
        Locale::ZhTw => Some(load_catalog(
            &ZH_TW_CATALOG,
            include_str!("locales/zh-TW.json"),
            "zh-TW",
        )),
        Locale::Ko => Some(load_catalog(
            &KO_CATALOG,
            include_str!("locales/ko.json"),
            "ko",
        )),
        Locale::Pt => Some(load_catalog(
            &PT_CATALOG,
            include_str!("locales/pt.json"),
            "pt",
        )),
        Locale::Fr => Some(load_catalog(
            &FR_CATALOG,
            include_str!("locales/fr.json"),
            "fr",
        )),
        Locale::De => Some(load_catalog(
            &DE_CATALOG,
            include_str!("locales/de.json"),
            "de",
        )),
        Locale::Es => Some(load_catalog(
            &ES_CATALOG,
            include_str!("locales/es.json"),
            "es",
        )),
        Locale::It => Some(load_catalog(
            &IT_CATALOG,
            include_str!("locales/it.json"),
            "it",
        )),
    }
}

fn load_catalog(
    lock: &'static OnceLock<LocaleCatalog>,
    source: &'static str,
    locale_code: &'static str,
) -> &'static LocaleCatalog {
    lock.get_or_init(|| {
        serde_json::from_str(source)
            .unwrap_or_else(|err| panic!("failed to parse {locale_code} locale catalog: {err}"))
    })
}
