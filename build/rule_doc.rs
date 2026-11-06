#[derive(Debug, Clone)]
pub(super) struct UpstreamRuleDocument {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) aliases: Vec<String>,
    pub(super) summary: String,
    pub(super) properties: Vec<UpstreamRuleProperty>,
    pub(super) fixable: bool,
}

#[derive(Debug, Clone)]
pub(super) struct UpstreamRuleProperty {
    pub(super) key: String,
    pub(super) value_type: String,
    pub(super) default_value: Option<String>,
    pub(super) description: String,
    pub(super) values: Vec<String>,
}

pub(super) fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
    let parsed = super::common::parse_document(source, None, |property, description| {
        UpstreamRuleProperty {
            key: property.key,
            value_type: property.value_type,
            default_value: property.default_value,
            description: description.to_owned(),
            values: property.values,
        }
    })?;
    Ok(UpstreamRuleDocument {
        id: parsed.id,
        name: parsed.name,
        aliases: parsed.aliases,
        summary: parsed.summary,
        properties: parsed.properties,
        fixable: parsed.fixable,
    })
}
