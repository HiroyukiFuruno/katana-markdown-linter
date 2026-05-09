use crate::rules::markdown::MarkdownRule;

pub(super) fn build_user_configurable_rules() -> Vec<Box<dyn MarkdownRule>> {
    use crate::rules::markdown::stubs::*;
    let mut all: Vec<Box<dyn MarkdownRule>> = super::official::build_official_rules()
        .into_iter()
        .filter(|rule| rule.official_meta().is_some())
        .collect();

    let stubs: Vec<Box<dyn MarkdownRule>> = vec![
        Box::new(RuleMD001),
        Box::new(RuleMD003),
        Box::new(RuleMD004),
        Box::new(RuleMD011),
        Box::new(RuleMD012),
        Box::new(RuleMD013),
        Box::new(RuleMD014),
        Box::new(RuleMD020),
        Box::new(RuleMD021),
        Box::new(RuleMD022),
        Box::new(RuleMD023),
        Box::new(RuleMD024),
        Box::new(RuleMD025),
        Box::new(RuleMD026),
        Box::new(RuleMD027),
        Box::new(RuleMD028),
        Box::new(RuleMD029),
        Box::new(RuleMD030),
        Box::new(RuleMD031),
        Box::new(RuleMD032),
        Box::new(RuleMD033),
        Box::new(RuleMD034),
        Box::new(RuleMD035),
        Box::new(RuleMD036),
        Box::new(RuleMD040),
        Box::new(RuleMD041),
        Box::new(RuleMD042),
        Box::new(RuleMD043),
        Box::new(RuleMD044),
        Box::new(RuleMD045),
        Box::new(RuleMD046),
        Box::new(RuleMD047),
        Box::new(RuleMD048),
        Box::new(RuleMD049),
        Box::new(RuleMD050),
        Box::new(RuleMD051),
        Box::new(RuleMD052),
        Box::new(RuleMD053),
        Box::new(RuleMD054),
        Box::new(RuleMD055),
    ];

    let existing_ids: std::collections::HashSet<&str> = all.iter().map(|rule| rule.id()).collect();
    for stub in stubs {
        if !existing_ids.contains(stub.id()) {
            all.push(stub);
        }
    }

    all.sort_by_key(|rule| rule.id());
    all
}
