use crate::types::{FixResult, LintResult};

pub fn apply(_results: &[LintResult], content: &str) -> FixResult {
    FixResult {
        content: content.to_owned(),
        applied_fixes: 0,
    }
}
