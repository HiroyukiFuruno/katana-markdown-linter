use ignore::{WalkBuilder, WalkState};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

pub(super) fn collect_markdown_files(
    dir: &Path,
    respect_gitignore: bool,
) -> Result<Vec<PathBuf>, String> {
    let (tx, rx) = mpsc::channel();
    let walker = WalkBuilder::new(dir)
        .hidden(false)
        .parents(true)
        .ignore(respect_gitignore)
        .git_ignore(respect_gitignore)
        .git_global(respect_gitignore)
        .git_exclude(respect_gitignore)
        .follow_links(false)
        .require_git(false)
        .build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |entry| {
            match entry {
                Ok(entry) => send_markdown_path(&tx, entry.into_path()),
                Err(error) => {
                    let _ = tx.send(Err(error.to_string()));
                }
            }
            WalkState::Continue
        })
    });
    drop(tx);

    let mut paths = Vec::new();
    for result in rx {
        match result {
            Ok(path) => paths.push(path),
            Err(error) => return Err(format!("{}: {error}", dir.display())),
        }
    }
    paths.sort();
    Ok(paths)
}

fn send_markdown_path(tx: &mpsc::Sender<Result<PathBuf, String>>, path: PathBuf) {
    if path.is_file() && is_markdown_file(&path) {
        let _ = tx.send(Ok(path));
    }
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            let extension = extension.to_ascii_lowercase();
            extension == "md" || extension == "markdown"
        })
        .unwrap_or(false)
}
