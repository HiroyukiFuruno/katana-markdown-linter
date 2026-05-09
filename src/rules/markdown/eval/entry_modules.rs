use super::RuleEntry;
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
use crate::rules::markdown::{
    DocumentContext, HeadingStructureRule, ListIndentRule, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};

macro_rules! rule_entry {
    ($module:ident: $rule:ident, $id:literal) => {
        pub(in crate::rules::markdown::eval) mod $module {
            use super::*;

            pub(in crate::rules::markdown::eval) fn official_meta() -> Option<OfficialRuleMeta> {
                $rule.official_meta()
            }

            pub(in crate::rules::markdown::eval) fn evaluate_context(
                ctx: &DocumentContext<'_>,
                config: Option<&crate::RuleConfig>,
            ) -> Vec<MarkdownDiagnostic> {
                $rule.evaluate_context(ctx, config)
            }

            pub(in crate::rules::markdown::eval) const ENTRY: RuleEntry = RuleEntry {
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
