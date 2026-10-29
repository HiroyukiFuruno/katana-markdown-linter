use crate::FixSafety;

const SYNTAX_RULES: &[&str] = &["MD018", "MD019", "MD020", "MD021", "MD023"];
const LAYOUT_RULES: &[&str] = &["MD012", "MD022", "MD031", "MD032", "MD047", "MD058"];

#[derive(Clone, Copy)]
pub(super) enum FixStage {
    Syntax,
    Content,
    Layout,
}

impl FixStage {
    pub(super) const ALL: [Self; 3] = [Self::Syntax, Self::Content, Self::Layout];

    pub(super) fn contains(self, rule_id: &str) -> bool {
        match self {
            Self::Syntax => SYNTAX_RULES.contains(&rule_id),
            Self::Content => !SYNTAX_RULES.contains(&rule_id) && !LAYOUT_RULES.contains(&rule_id),
            Self::Layout => LAYOUT_RULES.contains(&rule_id),
        }
    }
}

pub(super) fn preserves_meaning(rule_id: &str, safety: FixSafety) -> bool {
    safety == FixSafety::Safe && matches!(rule_id, "MD049" | "MD050")
}
