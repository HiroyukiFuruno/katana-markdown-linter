use crate::rules::markdown::DiagnosticSeverity;

regex_rule!(
    RuleMD009,
    "MD009",
    "Trailing spaces",
    r" \s+$",
    DiagnosticSeverity::Warning,
    &[
        crate::rule_prop!(Number, "br_spaces", "Spaces for line break", "2"),
        crate::rule_prop!(Boolean, "code_blocks", "Include code blocks", "false"),
        crate::rule_prop!(
            Boolean,
            "list_item_empty_lines",
            "Allow spaces for empty lines in list items",
            "false"
        ),
        crate::rule_prop!(Boolean, "strict", "Include unnecessary breaks", "false"),
    ]
);
regex_rule!(
    RuleMD010,
    "MD010",
    "Hard tabs",
    r"\t",
    DiagnosticSeverity::Warning,
    &[
        crate::rule_prop!(Boolean, "code_blocks", "Include code blocks", "true"),
        crate::rule_prop!(
            StringArray,
            "ignore_code_languages",
            "Fenced code languages to ignore",
            "[]"
        ),
        crate::rule_prop!(
            Number,
            "spaces_per_tab",
            "Number of spaces for each hard tab",
            "1"
        ),
    ]
);
regex_rule!(
    RuleMD020,
    "MD020",
    "No space inside blockquote marker",
    r"^>\S",
    DiagnosticSeverity::Warning,
    &[]
);
regex_rule!(
    RuleMD021,
    "MD021",
    "Multiple spaces after blockquote marker",
    r"^>\s{2,}\S",
    DiagnosticSeverity::Warning,
    &[]
);
