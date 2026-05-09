use super::types::{CliReport, FileReport};
use std::path::Path;

pub(super) fn fixed_count_message(report: &CliReport, file: &FileReport) -> (&'static str, String) {
    if report.command == "fmt" {
        return (
            "format.formatted_count",
            format!(
                "{}: formatted {} operation{}",
                file.path,
                file.applied_fixes,
                plural(file.applied_fixes)
            ),
        );
    }
    (
        "fix.fixed_count",
        format!(
            "{}: fixed {} issue{}",
            file.path,
            file.applied_fixes,
            plural(file.applied_fixes)
        ),
    )
}

pub(in crate::cli) fn print_diff(path: &Path, before: &str, after: &str) {
    if before == after {
        return;
    }
    println!("--- {}", path.display());
    println!("+++ {}", path.display());
    println!("@@");
    println!("-{}", before);
    println!("+{}", after);
}

pub(in crate::cli) fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}
