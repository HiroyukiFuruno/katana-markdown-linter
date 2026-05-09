use std::fs;
use std::path::Path;

pub(super) fn get_rule_documentation_from_dir(
    rule_id: &str,
    locale: crate::Locale,
    base_dir: &Path,
) -> Result<String, String> {
    let id = rule_id.to_lowercase();
    let file_name = format!("{}.md", id);

    let path = match locale {
        crate::Locale::En => base_dir.join(&file_name),
        _ => base_dir.join(locale.code()).join(&file_name),
    };

    fs::read_to_string(&path).map_err(|err| {
        format!(
            "failed to read documentation for {rule_id} from {}: {err}",
            path.display()
        )
    })
}
