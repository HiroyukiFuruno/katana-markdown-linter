use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigErrorKind {
    InvalidRoot,
    UnknownRule,
    UnknownProperty,
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },
    InvalidEnumValue {
        allowed: Vec<&'static str>,
        actual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub rule_id: Option<String>,
    pub property: Option<String>,
    pub kind: ConfigErrorKind,
    pub message: String,
}

impl ConfigError {
    pub(super) fn new(
        rule_id: Option<String>,
        property: Option<String>,
        kind: ConfigErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id,
            property,
            kind,
            message: message.into(),
        }
    }

    pub fn kind_code(&self) -> &'static str {
        self.kind.code()
    }

    pub fn message_id(&self) -> &'static str {
        self.kind.message_id()
    }

    pub fn message_params(&self) -> crate::i18n::MessageParams {
        let mut params = crate::i18n::MessageParams::new();
        if let Some(rule_id) = &self.rule_id {
            params.insert("rule_id".to_string(), rule_id.clone());
        }
        if let Some(property) = &self.property {
            params.insert("property".to_string(), property.clone());
        }
        self.kind.fill_message_params(&mut params);
        params.insert("message".to_string(), self.message.clone());
        params
    }

    pub fn localized_message(&self, locale: crate::i18n::Locale) -> String {
        crate::i18n::MessageCatalog::render_message(
            locale,
            self.message_id(),
            &self.message_params(),
            &self.to_string(),
        )
    }

    pub fn expected(&self) -> Option<&'static str> {
        match &self.kind {
            ConfigErrorKind::InvalidType { expected, .. } => Some(expected),
            _ => None,
        }
    }

    pub fn actual(&self) -> Option<&str> {
        match &self.kind {
            ConfigErrorKind::InvalidType { actual, .. } => Some(actual),
            ConfigErrorKind::InvalidEnumValue { actual, .. } => Some(actual),
            _ => None,
        }
    }

    pub fn allowed(&self) -> Vec<&'static str> {
        match &self.kind {
            ConfigErrorKind::InvalidEnumValue { allowed, .. } => allowed.clone(),
            _ => Vec::new(),
        }
    }
}

impl ConfigErrorKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidRoot => "invalid_root",
            Self::UnknownRule => "unknown_rule",
            Self::UnknownProperty => "unknown_property",
            Self::InvalidType { .. } => "invalid_type",
            Self::InvalidEnumValue { .. } => "invalid_enum_value",
        }
    }

    pub fn message_id(&self) -> &'static str {
        match self {
            Self::InvalidRoot => "config.invalid_root",
            Self::UnknownRule => "config.unknown_rule",
            Self::UnknownProperty => "config.unknown_property",
            Self::InvalidType { .. } => "config.invalid_type",
            Self::InvalidEnumValue { .. } => "config.invalid_enum_value",
        }
    }

    fn fill_message_params(&self, params: &mut crate::i18n::MessageParams) {
        match self {
            Self::InvalidType { expected, actual } => {
                params.insert("expected".to_string(), (*expected).to_string());
                params.insert("actual".to_string(), (*actual).to_string());
            }
            Self::InvalidEnumValue { allowed, actual } => {
                params.insert("allowed".to_string(), allowed.join(", "));
                params.insert("actual".to_string(), actual.clone());
            }
            Self::InvalidRoot | Self::UnknownRule | Self::UnknownProperty => {}
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.rule_id, &self.property) {
            (Some(rule_id), Some(property)) => {
                write!(formatter, "{rule_id}.{property}: {}", self.message)
            }
            (Some(rule_id), None) => write!(formatter, "{rule_id}: {}", self.message),
            _ => formatter.write_str(&self.message),
        }
    }
}
