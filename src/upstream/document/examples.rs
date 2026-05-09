use super::types::UpstreamRuleExample;

pub(super) fn parse_fenced_examples(source: &str) -> Vec<UpstreamRuleExample> {
    let mut examples = Vec::new();
    let mut in_fence = false;
    let mut language = None;
    let mut content = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("```") else {
            if in_fence {
                content.push_str(line);
                content.push('\n');
            }
            continue;
        };

        if in_fence {
            examples.push(UpstreamRuleExample {
                language: language.take(),
                content: content.trim_end_matches('\n').to_string(),
            });
            content.clear();
            in_fence = false;
            continue;
        }

        let lang = rest.trim();
        language = if lang.is_empty() {
            None
        } else {
            Some(lang.to_string())
        };
        in_fence = true;
    }
    examples
}
