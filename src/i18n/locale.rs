#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Locale {
    En,
    Ja,
    ZhCn,
    ZhTw,
    Ko,
    Pt,
    Fr,
    De,
    Es,
    It,
}

impl Locale {
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ja => "ja",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
            Self::Ko => "ko",
            Self::Pt => "pt",
            Self::Fr => "fr",
            Self::De => "de",
            Self::Es => "es",
            Self::It => "it",
        }
    }

    pub fn resolve(explicit: Option<&str>) -> Result<Self, LocaleError> {
        Self::resolve_with(explicit, |key| std::env::var(key).ok())
    }

    pub(crate) fn resolve_with(
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
        let mut parts = normalized.split('-');
        let primary = parts.next().unwrap_or(normalized.as_str());
        let secondary = parts.next();
        match primary {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            "zh" => match secondary {
                Some("tw" | "hk" | "mo" | "hant") => Some(Self::ZhTw),
                _ => Some(Self::ZhCn),
            },
            "ko" => Some(Self::Ko),
            "pt" => Some(Self::Pt),
            "fr" => Some(Self::Fr),
            "de" => Some(Self::De),
            "es" => Some(Self::Es),
            "it" => Some(Self::It),
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
    &SUPPORTED_LOCALES
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleError {
    pub locale: String,
}

impl std::fmt::Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unsupported locale: {} (supported: {})",
            self.locale,
            SUPPORTED_LOCALE_CODES.join(", ")
        )
    }
}

const SUPPORTED_LOCALES: [Locale; 10] = [
    Locale::En,
    Locale::Ja,
    Locale::ZhCn,
    Locale::ZhTw,
    Locale::Ko,
    Locale::Pt,
    Locale::Fr,
    Locale::De,
    Locale::Es,
    Locale::It,
];

const SUPPORTED_LOCALE_CODES: [&str; 10] = [
    "en", "ja", "zh-CN", "zh-TW", "ko", "pt", "fr", "de", "es", "it",
];
