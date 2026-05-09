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
use crate::rules::markdown::{HeadingStructureRule, ListIndentRule, MarkdownRule};

pub(super) fn build_official_rules() -> Vec<Box<dyn MarkdownRule>> {
    vec![
        Box::new(HeadingStructureRule),
        Box::new(HeadingStyleRule),
        Box::new(BlanksAroundHeadingsRule),
        Box::new(HeadingStartLeftRule),
        Box::new(SingleH1Rule),
        Box::new(NoTrailingPunctuationRule),
        Box::new(NoTrailingSpacesRule),
        Box::new(NoHardTabsRule),
        Box::new(NoReversedLinksRule),
        Box::new(NoMultipleBlanksRule),
        Box::new(LineLengthRule),
        Box::new(DollarSignsBeforeCommandsRule),
        Box::new(NoMissingSpaceAtxRule),
        Box::new(NoMultipleSpaceAtxRule),
        Box::new(NoMissingSpaceClosedAtxRule),
        Box::new(NoMultipleSpaceClosedAtxRule),
        Box::new(NoBareUrlsRule),
        Box::new(SpacesInEmphasisRule),
        Box::new(NoSpaceInCodeRule),
        Box::new(NoSpacesInLinksRule),
        Box::new(NoMultipleSpaceBlockquoteRule),
        Box::new(NoBlanksBlockquoteRule),
        Box::new(NoDuplicateHeadingRule),
        Box::new(BlanksAroundFencesRule),
        Box::new(ListMarkerSpaceRule),
        Box::new(SingleTrailingNewlineRule),
        Box::new(NoInlineHtmlRule),
        Box::new(FencedCodeLanguageRule),
        Box::new(FirstLineHeadingRule),
        Box::new(NoEmptyLinksRule),
        Box::new(TableColumnCountRule),
        Box::new(TableSpacingRule),
        Box::new(ProhibitedLinkTextRule),
        Box::new(TableColumnStyleRule),
        Box::new(RequiredHeadingsRule),
        Box::new(ProperNamesRule),
        Box::new(ListIndentRule),
        Box::new(UnorderedListIndentRule),
        Box::new(UlStyleRule),
        Box::new(OlPrefixRule),
        Box::new(BlanksAroundListsRule),
        Box::new(HrStyleRule),
        Box::new(NoEmphasisAsHeadingRule),
        Box::new(NoAltTextRule),
        Box::new(CodeBlockStyleRule),
        Box::new(CodeFenceStyleRule),
        Box::new(EmphasisStyleRule),
        Box::new(StrongStyleRule),
        Box::new(LinkFragmentsRule),
        Box::new(ReferenceLinksImagesRule),
        Box::new(LinkDefinitionsRule),
        Box::new(LinkStyleRule),
        Box::new(TablePipeStyleRule),
    ]
}
