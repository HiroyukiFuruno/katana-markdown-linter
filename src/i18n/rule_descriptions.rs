use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

use super::Locale;

pub(super) fn rule_description(locale: Locale, rule_id: &str) -> Option<&'static str> {
    catalog(locale)?
        .rule_descriptions
        .get(rule_id)
        .map(String::as_str)
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
    let (lock, source) = match locale {
        Locale::En => return None,
        Locale::Ja => (&JA_CATALOG, include_str!("locales/ja.json")),
        Locale::ZhCn => (&ZH_CN_CATALOG, include_str!("locales/zh-CN.json")),
        Locale::ZhTw => (&ZH_TW_CATALOG, include_str!("locales/zh-TW.json")),
        Locale::Ko => (&KO_CATALOG, include_str!("locales/ko.json")),
        Locale::Pt => (&PT_CATALOG, include_str!("locales/pt.json")),
        Locale::Fr => (&FR_CATALOG, include_str!("locales/fr.json")),
        Locale::De => (&DE_CATALOG, include_str!("locales/de.json")),
        Locale::Es => (&ES_CATALOG, include_str!("locales/es.json")),
        Locale::It => (&IT_CATALOG, include_str!("locales/it.json")),
    };
    Some(load_catalog(lock, source, locale.code()))
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
