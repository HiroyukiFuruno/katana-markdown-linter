use crate::rules::markdown::DocumentContext;
use std::collections::{HashMap, HashSet};

pub(super) fn heading_fragments(ctx: &DocumentContext<'_>) -> HashSet<String> {
    let mut counts = HashMap::<String, usize>::new();
    let mut fragments = HashSet::new();

    fragments.extend(html_defined_fragments(ctx));

    for heading in ctx.headings() {
        if let Some(anchor) = custom_heading_anchor(heading.text) {
            fragments.insert(anchor.to_string());
        }
        let base = github_heading_slug(heading.text);
        if base.is_empty() {
            continue;
        }
        let count = counts.entry(base.clone()).or_insert(0);
        let fragment = if *count == 0 {
            base.clone()
        } else {
            format!("{base}-{count}")
        };
        *count += 1;
        fragments.insert(fragment);
    }

    fragments
}

fn github_heading_slug(heading: &str) -> String {
    let heading = heading
        .trim_end_matches('#')
        .trim()
        .split_once("{#")
        .map_or(heading.trim(), |(before, _)| before.trim());
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in heading.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() || ch == '-' || ch == '_' {
            slug.push(ch);
            previous_dash = false;
        } else if ch.is_whitespace() && !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn custom_heading_anchor(heading: &str) -> Option<&str> {
    let start = heading.rfind("{#")?;
    let after_start = &heading[start + 2..];
    let end = after_start.find('}')?;
    let anchor = &after_start[..end];
    if !anchor.is_empty()
        && anchor
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        Some(anchor)
    } else {
        None
    }
}

fn html_defined_fragments(ctx: &DocumentContext<'_>) -> Vec<String> {
    let mut fragments = Vec::new();
    for element in ctx.inline_html_elements() {
        if let Some(id) = element.attribute_value("id") {
            fragments.push(id.to_string());
        }
        if element.name.eq_ignore_ascii_case("a") {
            if let Some(name) = element.attribute_value("name") {
                fragments.push(name.to_string());
            }
        }
    }
    fragments
}
