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
    RuleMD011,
    "MD011",
    "Reversed link syntax",
    r"\]\([^)]+?\[",
    DiagnosticSeverity::Warning,
    &[]
);
regex_rule!(
    RuleMD013,
    "MD013",
    "Line length",
    r"^.{81,}$",
    DiagnosticSeverity::Warning,
    &[
        crate::rule_prop!(Number, "line_length", "Number of characters", "80"),
        crate::rule_prop!(
            Number,
            "heading_line_length",
            "Number of characters for headings",
            "80"
        ),
        crate::rule_prop!(
            Number,
            "code_block_line_length",
            "Number of characters for code blocks",
            "80"
        ),
        crate::rule_prop!(Boolean, "code_blocks", "Include code blocks", "true"),
        crate::rule_prop!(Boolean, "tables", "Include tables", "true"),
        crate::rule_prop!(Boolean, "headings", "Include headings", "true"),
        crate::rule_prop!(Boolean, "strict", "Strict length checking", "false"),
        crate::rule_prop!(Boolean, "stern", "Stern length checking", "false"),
    ]
);
regex_rule!(
    RuleMD018,
    "MD018",
    "No space after hash on atx style heading",
    r"^#+[^\s#]",
    DiagnosticSeverity::Error,
    &[]
);
regex_rule!(
    RuleMD014,
    "MD014",
    "Dollar signs used before commands without spaces",
    r"^\s*\$[^\s]",
    DiagnosticSeverity::Warning,
    &[]
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
regex_rule!(
    RuleMD019,
    "MD019",
    "Multiple spaces after hash on atx style heading",
    r"^#+\s{2,}[^\s#]",
    DiagnosticSeverity::Warning,
    &[]
);
regex_rule!(
    RuleMD034,
    "MD034",
    "No bare URLs",
    r"https?://\S+",
    DiagnosticSeverity::Warning,
    &[]
);
regex_rule!(
    RuleMD039,
    "MD039",
    "Spaces inside link text",
    r"\[\s+[^\]]+\s+\]",
    DiagnosticSeverity::Warning,
    &[]
);
