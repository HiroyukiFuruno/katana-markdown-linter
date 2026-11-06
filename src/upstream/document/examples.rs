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
            finish_example(&mut examples, &mut language, &mut content);
        } else {
            language = fence_language(rest);
        }
        in_fence = !in_fence;
    }
    examples
}

fn finish_example(
    examples: &mut Vec<UpstreamRuleExample>,
    language: &mut Option<String>,
    content: &mut String,
) {
    examples.push(UpstreamRuleExample {
        language: language.take(),
        content: content.trim_end_matches('\n').to_string(),
    });
    content.clear();
}

fn fence_language(rest: &str) -> Option<String> {
    let lang = rest.trim();
    (!lang.is_empty()).then(|| lang.to_string())
}
