use crate::rules::markdown::HeadingStructureRule;
use crate::rules::markdown::ListIndentRule;

use crate::rules::markdown::rules::blockquote::*;
use crate::rules::markdown::rules::content::*;
use crate::rules::markdown::rules::content_ext::*;
use crate::rules::markdown::rules::fences::*;
use crate::rules::markdown::rules::heading::*;
use crate::rules::markdown::rules::heading_duplicates::*;
use crate::rules::markdown::rules::heading_ext::*;
use crate::rules::markdown::rules::heading_style::*;
use crate::rules::markdown::rules::image::*;
use crate::rules::markdown::rules::list::*;
use crate::rules::markdown::rules::list_ext::*;
use crate::rules::markdown::rules::list_indent::*;
use crate::rules::markdown::rules::list_spacing::*;
use crate::rules::markdown::rules::md009::*;
use crate::rules::markdown::rules::md010::*;
use crate::rules::markdown::rules::md011::*;
use crate::rules::markdown::rules::md013::*;
use crate::rules::markdown::rules::md014::*;
use crate::rules::markdown::rules::md018::*;
use crate::rules::markdown::rules::md019::*;
use crate::rules::markdown::rules::md020::*;
use crate::rules::markdown::rules::md021::*;
use crate::rules::markdown::rules::md033::*;
use crate::rules::markdown::rules::md034::*;
use crate::rules::markdown::rules::md039::*;
use crate::rules::markdown::rules::md043::*;
use crate::rules::markdown::rules::md044::*;
use crate::rules::markdown::rules::md046::*;
use crate::rules::markdown::rules::md048::*;
use crate::rules::markdown::rules::md049::*;
use crate::rules::markdown::rules::md050::*;
use crate::rules::markdown::rules::md051::*;
use crate::rules::markdown::rules::md052::*;
use crate::rules::markdown::rules::md053::*;
use crate::rules::markdown::rules::md054::*;
use crate::rules::markdown::rules::md055::*;
use crate::rules::markdown::rules::md056::*;
use crate::rules::markdown::rules::md058::*;
use crate::rules::markdown::rules::md059::*;
use crate::rules::markdown::rules::md060::*;
use crate::rules::markdown::rules::spaces_in_code::NoSpaceInCodeRule;
use crate::rules::markdown::rules::spaces_in_emphasis::SpacesInEmphasisRule;
use crate::rules::markdown::rules::style::*;
use crate::rules::markdown::rules::whitespace::*;
use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta};
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct MarkdownLinterOps;

#[derive(Clone, Copy)]
pub struct RuleEntry {
    id: &'static str,
    official_meta: fn() -> Option<OfficialRuleMeta>,
    evaluate_context:
        fn(&DocumentContext<'_>, Option<&crate::RuleConfig>) -> Vec<MarkdownDiagnostic>,
}

impl RuleEntry {
    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn official_meta(&self) -> Option<OfficialRuleMeta> {
        (self.official_meta)()
    }

    pub fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&crate::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        (self.evaluate_context)(ctx, config)
    }
}

macro_rules! rule_entry {
    ($module:ident: $rule:ident, $id:literal) => {
        mod $module {
            use super::*;

            pub fn official_meta() -> Option<OfficialRuleMeta> {
                $rule.official_meta()
            }

            pub fn evaluate_context(
                ctx: &DocumentContext<'_>,
                config: Option<&crate::RuleConfig>,
            ) -> Vec<MarkdownDiagnostic> {
                $rule.evaluate_context(ctx, config)
            }

            pub const ENTRY: RuleEntry = RuleEntry {
                id: $id,
                official_meta,
                evaluate_context,
            };
        }
    };
}

rule_entry!(heading_structure_rule_entry: HeadingStructureRule, "MD001");
rule_entry!(heading_style_rule_entry: HeadingStyleRule, "MD003");
rule_entry!(blanks_around_headings_rule_entry: BlanksAroundHeadingsRule, "MD022");
rule_entry!(heading_start_left_rule_entry: HeadingStartLeftRule, "MD023");
rule_entry!(single_h1_rule_entry: SingleH1Rule, "MD025");
rule_entry!(no_trailing_punctuation_rule_entry: NoTrailingPunctuationRule, "MD026");
rule_entry!(no_trailing_spaces_rule_entry: NoTrailingSpacesRule, "MD009");
rule_entry!(no_hard_tabs_rule_entry: NoHardTabsRule, "MD010");
rule_entry!(no_reversed_links_rule_entry: NoReversedLinksRule, "MD011");
rule_entry!(no_multiple_blanks_rule_entry: NoMultipleBlanksRule, "MD012");
rule_entry!(line_length_rule_entry: LineLengthRule, "MD013");
rule_entry!(dollar_signs_before_commands_rule_entry: DollarSignsBeforeCommandsRule, "MD014");
rule_entry!(no_missing_space_atx_rule_entry: NoMissingSpaceAtxRule, "MD018");
rule_entry!(no_multiple_space_atx_rule_entry: NoMultipleSpaceAtxRule, "MD019");
rule_entry!(no_missing_space_closed_atx_rule_entry: NoMissingSpaceClosedAtxRule, "MD020");
rule_entry!(
    no_multiple_space_closed_atx_rule_entry: NoMultipleSpaceClosedAtxRule,
    "MD021"
);
rule_entry!(no_bare_urls_rule_entry: NoBareUrlsRule, "MD034");
rule_entry!(spaces_in_emphasis_rule_entry: SpacesInEmphasisRule, "MD037");
rule_entry!(no_space_in_code_rule_entry: NoSpaceInCodeRule, "MD038");
rule_entry!(no_spaces_in_links_rule_entry: NoSpacesInLinksRule, "MD039");
rule_entry!(no_multiple_space_blockquote_rule_entry: NoMultipleSpaceBlockquoteRule, "MD027");
rule_entry!(no_blanks_blockquote_rule_entry: NoBlanksBlockquoteRule, "MD028");
rule_entry!(no_duplicate_heading_rule_entry: NoDuplicateHeadingRule, "MD024");
rule_entry!(blanks_around_fences_rule_entry: BlanksAroundFencesRule, "MD031");
rule_entry!(list_marker_space_rule_entry: ListMarkerSpaceRule, "MD030");
rule_entry!(single_trailing_newline_rule_entry: SingleTrailingNewlineRule, "MD047");
rule_entry!(no_inline_html_rule_entry: NoInlineHtmlRule, "MD033");
rule_entry!(fenced_code_language_rule_entry: FencedCodeLanguageRule, "MD040");
rule_entry!(first_line_heading_rule_entry: FirstLineHeadingRule, "MD041");
rule_entry!(no_empty_links_rule_entry: NoEmptyLinksRule, "MD042");
rule_entry!(table_column_count_rule_entry: TableColumnCountRule, "MD056");
rule_entry!(table_spacing_rule_entry: TableSpacingRule, "MD058");
rule_entry!(prohibited_link_text_rule_entry: ProhibitedLinkTextRule, "MD059");
rule_entry!(table_column_style_rule_entry: TableColumnStyleRule, "MD060");
rule_entry!(required_headings_rule_entry: RequiredHeadingsRule, "MD043");
rule_entry!(proper_names_rule_entry: ProperNamesRule, "MD044");
rule_entry!(list_indent_rule_entry: ListIndentRule, "MD005");
rule_entry!(unordered_list_indent_rule_entry: UnorderedListIndentRule, "MD007");
rule_entry!(ul_style_rule_entry: UlStyleRule, "MD004");
rule_entry!(ol_prefix_rule_entry: OlPrefixRule, "MD029");
rule_entry!(blanks_around_lists_rule_entry: BlanksAroundListsRule, "MD032");
rule_entry!(hr_style_rule_entry: HrStyleRule, "MD035");
rule_entry!(no_emphasis_as_heading_rule_entry: NoEmphasisAsHeadingRule, "MD036");
rule_entry!(no_alt_text_rule_entry: NoAltTextRule, "MD045");
rule_entry!(code_block_style_rule_entry: CodeBlockStyleRule, "MD046");
rule_entry!(code_fence_style_rule_entry: CodeFenceStyleRule, "MD048");
rule_entry!(emphasis_style_rule_entry: EmphasisStyleRule, "MD049");
rule_entry!(strong_style_rule_entry: StrongStyleRule, "MD050");
rule_entry!(link_fragments_rule_entry: LinkFragmentsRule, "MD051");
rule_entry!(reference_links_images_rule_entry: ReferenceLinksImagesRule, "MD052");
rule_entry!(link_definitions_rule_entry: LinkDefinitionsRule, "MD053");
rule_entry!(link_style_rule_entry: LinkStyleRule, "MD054");
rule_entry!(table_pipe_style_rule_entry: TablePipeStyleRule, "MD055");

static OFFICIAL_RULES: &[RuleEntry] = &[
    heading_structure_rule_entry::ENTRY,
    heading_style_rule_entry::ENTRY,
    blanks_around_headings_rule_entry::ENTRY,
    heading_start_left_rule_entry::ENTRY,
    single_h1_rule_entry::ENTRY,
    no_trailing_punctuation_rule_entry::ENTRY,
    no_trailing_spaces_rule_entry::ENTRY,
    no_hard_tabs_rule_entry::ENTRY,
    no_reversed_links_rule_entry::ENTRY,
    no_multiple_blanks_rule_entry::ENTRY,
    line_length_rule_entry::ENTRY,
    dollar_signs_before_commands_rule_entry::ENTRY,
    no_missing_space_atx_rule_entry::ENTRY,
    no_multiple_space_atx_rule_entry::ENTRY,
    no_missing_space_closed_atx_rule_entry::ENTRY,
    no_multiple_space_closed_atx_rule_entry::ENTRY,
    no_bare_urls_rule_entry::ENTRY,
    spaces_in_emphasis_rule_entry::ENTRY,
    no_space_in_code_rule_entry::ENTRY,
    no_spaces_in_links_rule_entry::ENTRY,
    no_multiple_space_blockquote_rule_entry::ENTRY,
    no_blanks_blockquote_rule_entry::ENTRY,
    no_duplicate_heading_rule_entry::ENTRY,
    blanks_around_fences_rule_entry::ENTRY,
    list_marker_space_rule_entry::ENTRY,
    single_trailing_newline_rule_entry::ENTRY,
    no_inline_html_rule_entry::ENTRY,
    fenced_code_language_rule_entry::ENTRY,
    first_line_heading_rule_entry::ENTRY,
    no_empty_links_rule_entry::ENTRY,
    table_column_count_rule_entry::ENTRY,
    table_spacing_rule_entry::ENTRY,
    prohibited_link_text_rule_entry::ENTRY,
    table_column_style_rule_entry::ENTRY,
    required_headings_rule_entry::ENTRY,
    proper_names_rule_entry::ENTRY,
    list_indent_rule_entry::ENTRY,
    unordered_list_indent_rule_entry::ENTRY,
    ul_style_rule_entry::ENTRY,
    ol_prefix_rule_entry::ENTRY,
    blanks_around_lists_rule_entry::ENTRY,
    hr_style_rule_entry::ENTRY,
    no_emphasis_as_heading_rule_entry::ENTRY,
    no_alt_text_rule_entry::ENTRY,
    code_block_style_rule_entry::ENTRY,
    code_fence_style_rule_entry::ENTRY,
    emphasis_style_rule_entry::ENTRY,
    strong_style_rule_entry::ENTRY,
    link_fragments_rule_entry::ENTRY,
    reference_links_images_rule_entry::ENTRY,
    link_definitions_rule_entry::ENTRY,
    link_style_rule_entry::ENTRY,
    table_pipe_style_rule_entry::ENTRY,
];

static USER_CONFIGURABLE_RULES: &[RuleEntry] = &[
    heading_structure_rule_entry::ENTRY,
    heading_style_rule_entry::ENTRY,
    blanks_around_headings_rule_entry::ENTRY,
    heading_start_left_rule_entry::ENTRY,
    single_h1_rule_entry::ENTRY,
    no_trailing_punctuation_rule_entry::ENTRY,
    no_trailing_spaces_rule_entry::ENTRY,
    no_hard_tabs_rule_entry::ENTRY,
    no_reversed_links_rule_entry::ENTRY,
    no_multiple_blanks_rule_entry::ENTRY,
    line_length_rule_entry::ENTRY,
    dollar_signs_before_commands_rule_entry::ENTRY,
    no_missing_space_atx_rule_entry::ENTRY,
    no_multiple_space_atx_rule_entry::ENTRY,
    no_missing_space_closed_atx_rule_entry::ENTRY,
    no_multiple_space_closed_atx_rule_entry::ENTRY,
    no_bare_urls_rule_entry::ENTRY,
    spaces_in_emphasis_rule_entry::ENTRY,
    no_space_in_code_rule_entry::ENTRY,
    no_spaces_in_links_rule_entry::ENTRY,
    no_multiple_space_blockquote_rule_entry::ENTRY,
    no_blanks_blockquote_rule_entry::ENTRY,
    no_duplicate_heading_rule_entry::ENTRY,
    blanks_around_fences_rule_entry::ENTRY,
    list_marker_space_rule_entry::ENTRY,
    single_trailing_newline_rule_entry::ENTRY,
    no_inline_html_rule_entry::ENTRY,
    fenced_code_language_rule_entry::ENTRY,
    first_line_heading_rule_entry::ENTRY,
    no_empty_links_rule_entry::ENTRY,
    table_column_count_rule_entry::ENTRY,
    table_spacing_rule_entry::ENTRY,
    prohibited_link_text_rule_entry::ENTRY,
    table_column_style_rule_entry::ENTRY,
    required_headings_rule_entry::ENTRY,
    proper_names_rule_entry::ENTRY,
    list_indent_rule_entry::ENTRY,
    unordered_list_indent_rule_entry::ENTRY,
    ul_style_rule_entry::ENTRY,
    ol_prefix_rule_entry::ENTRY,
    blanks_around_lists_rule_entry::ENTRY,
    hr_style_rule_entry::ENTRY,
    no_emphasis_as_heading_rule_entry::ENTRY,
    no_alt_text_rule_entry::ENTRY,
    code_block_style_rule_entry::ENTRY,
    code_fence_style_rule_entry::ENTRY,
    emphasis_style_rule_entry::ENTRY,
    strong_style_rule_entry::ENTRY,
    link_fragments_rule_entry::ENTRY,
    reference_links_images_rule_entry::ENTRY,
    link_definitions_rule_entry::ENTRY,
    link_style_rule_entry::ENTRY,
    table_pipe_style_rule_entry::ENTRY,
];

impl MarkdownLinterOps {
    pub fn evaluate_all(
        file_path: &std::path::Path,
        content: &str,
        enabled: bool,
        severity_map: &std::collections::HashMap<
            &str,
            Option<crate::rules::markdown::DiagnosticSeverity>,
        >,
        rule_configs: &std::collections::HashMap<String, crate::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let mut diagnostics = Vec::new();

        if !enabled {
            return diagnostics;
        }

        let ctx = DocumentContext::new(file_path, content);
        for rule in Self::official_rules() {
            let rule_id = rule.id();
            let sev_opt = severity_map
                .get(rule_id)
                .copied()
                .unwrap_or(Some(crate::rules::markdown::DiagnosticSeverity::Warning));
            if let Some(severity) = sev_opt {
                let mut diags = rule.evaluate_context(&ctx, rule_configs.get(rule_id));
                for d in &mut diags {
                    d.severity = severity;
                }
                diagnostics.extend(diags);
            }
        }

        diagnostics
    }

    pub fn official_rules() -> &'static [RuleEntry] {
        OFFICIAL_RULES
    }

    pub fn get_official_rules() -> Vec<Box<dyn MarkdownRule>> {
        Self::build_official_rules()
    }

    fn build_official_rules() -> Vec<Box<dyn MarkdownRule>> {
        vec![
            /* WHY: MD001 — heading-increment (full impl in mod.rs) */
            Box::new(HeadingStructureRule),
            /* WHY: Heading rules */
            Box::new(HeadingStyleRule),          // MD003
            Box::new(BlanksAroundHeadingsRule),  // MD022
            Box::new(HeadingStartLeftRule),      // MD023
            Box::new(SingleH1Rule),              // MD025
            Box::new(NoTrailingPunctuationRule), // MD026
            /* WHY: Regex-based rules */
            Box::new(NoTrailingSpacesRule),          // MD009
            Box::new(NoHardTabsRule),                // MD010
            Box::new(NoReversedLinksRule),           // MD011
            Box::new(NoMultipleBlanksRule),          // MD012
            Box::new(LineLengthRule),                // MD013
            Box::new(DollarSignsBeforeCommandsRule), // MD014
            Box::new(NoMissingSpaceAtxRule),         // MD018
            Box::new(NoMultipleSpaceAtxRule),        // MD019
            Box::new(NoMissingSpaceClosedAtxRule),   // MD020
            Box::new(NoMultipleSpaceClosedAtxRule),  // MD021
            Box::new(NoBareUrlsRule),                // MD034
            Box::new(SpacesInEmphasisRule),          // MD037
            Box::new(NoSpaceInCodeRule),             // MD038
            Box::new(NoSpacesInLinksRule),           // MD039
            /* WHY: Blockquote rules */
            Box::new(NoMultipleSpaceBlockquoteRule), // MD027
            Box::new(NoBlanksBlockquoteRule),        // MD028
            /* WHY: Additional active rules */
            Box::new(NoDuplicateHeadingRule),    // MD024
            Box::new(BlanksAroundFencesRule),    // MD031
            Box::new(ListMarkerSpaceRule),       // MD030
            Box::new(SingleTrailingNewlineRule), // MD047
            /* WHY: Content rules */
            Box::new(NoInlineHtmlRule),       // MD033
            Box::new(FencedCodeLanguageRule), // MD040
            Box::new(FirstLineHeadingRule),   // MD041
            Box::new(NoEmptyLinksRule),       // MD042
            Box::new(TableColumnCountRule),   // MD056
            Box::new(TableSpacingRule),       // MD058
            Box::new(ProhibitedLinkTextRule), // MD059
            Box::new(TableColumnStyleRule),   // MD060
            Box::new(RequiredHeadingsRule),   // MD043
            Box::new(ProperNamesRule),        // MD044
            /* WHY: List rules */
            Box::new(ListIndentRule),          // MD005
            Box::new(UnorderedListIndentRule), // MD007
            Box::new(UlStyleRule),             // MD004
            Box::new(OlPrefixRule),            // MD029
            Box::new(BlanksAroundListsRule),   // MD032
            /* WHY: Style rules */
            Box::new(HrStyleRule),              // MD035
            Box::new(NoEmphasisAsHeadingRule),  // MD036
            Box::new(NoAltTextRule),            // MD045
            Box::new(CodeBlockStyleRule),       // MD046
            Box::new(CodeFenceStyleRule),       // MD048
            Box::new(EmphasisStyleRule),        // MD049
            Box::new(StrongStyleRule),          // MD050
            Box::new(LinkFragmentsRule),        // MD051
            Box::new(ReferenceLinksImagesRule), // MD052
            Box::new(LinkDefinitionsRule),      // MD053
            Box::new(LinkStyleRule),            // MD054
            Box::new(TablePipeStyleRule),       // MD055
        ]
    }

    /* WHY: User-configurable rules are those with official_meta (i.e., not internal-only).
     * This includes both actively evaluated rules AND stub rules (official rules that are not
     * yet fully implemented). All are shown in the settings UI so the user can configure
     * severity for when they become active. Rules are deduplicated by ID and sorted. */
    pub fn user_configurable_rules() -> &'static [RuleEntry] {
        USER_CONFIGURABLE_RULES
    }

    pub fn user_configurable_rule_meta_map() -> &'static HashMap<&'static str, OfficialRuleMeta> {
        static RULE_META_MAP: OnceLock<HashMap<&'static str, OfficialRuleMeta>> = OnceLock::new();
        RULE_META_MAP.get_or_init(|| {
            Self::user_configurable_rules()
                .iter()
                .filter_map(|rule| rule.official_meta().map(|meta| (meta.code, meta)))
                .collect()
        })
    }

    pub fn get_user_configurable_rules() -> Vec<Box<dyn MarkdownRule>> {
        Self::build_user_configurable_rules()
    }

    fn build_user_configurable_rules() -> Vec<Box<dyn MarkdownRule>> {
        use crate::rules::markdown::stubs::*;
        let mut all: Vec<Box<dyn MarkdownRule>> = Self::build_official_rules()
            .into_iter()
            .filter(|r| r.official_meta().is_some())
            .collect();

        /* WHY: Add stub rules that represent official markdownlint rules not yet implemented.
         * These are shown in the settings UI for forward-compatibility with .markdownlint.json. */
        let stubs: Vec<Box<dyn MarkdownRule>> = vec![
            Box::new(RuleMD001),
            Box::new(RuleMD003),
            Box::new(RuleMD004),
            Box::new(RuleMD007),
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

        /* WHY: Collect existing IDs first to avoid duplicating rules already in get_official_rules(). */
        let existing_ids: std::collections::HashSet<&str> = all.iter().map(|r| r.id()).collect();
        for stub in stubs {
            if !existing_ids.contains(stub.id()) {
                all.push(stub);
            }
        }

        all.sort_by_key(|r| r.id());
        all
    }
}
