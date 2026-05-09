use crate::types::RuleConfig;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TableStyle {
    Aligned,
    Compact,
    Tight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ConfiguredStyle {
    Any,
    Style(TableStyle),
}

pub(super) struct TableStyleOptions {
    pub(super) style: ConfiguredStyle,
    pub(super) aligned_delimiter: bool,
}

impl TableStyleOptions {
    pub(super) fn from_config(config: Option<&RuleConfig>) -> Self {
        let style = config
            .and_then(|config| config.properties.get("style"))
            .map(|style| match style.as_str() {
                "aligned" => ConfiguredStyle::Style(TableStyle::Aligned),
                "compact" => ConfiguredStyle::Style(TableStyle::Compact),
                "tight" => ConfiguredStyle::Style(TableStyle::Tight),
                _ => ConfiguredStyle::Any,
            })
            .unwrap_or(ConfiguredStyle::Any);
        let aligned_delimiter = config
            .and_then(|config| config.properties.get("aligned_delimiter"))
            .is_some_and(|value| value == "true");
        Self {
            style,
            aligned_delimiter,
        }
    }
}
