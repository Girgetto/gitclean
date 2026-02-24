use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

pub fn find_git_repos(start_path: &str, git_dirs: Arc<Mutex<Vec<PathBuf>>>) {
    let mut walker = WalkDir::new(start_path).follow_links(false).into_iter();
    loop {
        match walker.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => {
                if entry.file_type().is_dir() && entry.file_name() == ".git" {
                    git_dirs.lock().unwrap().push(entry.path().to_path_buf());
                    // Skip descending into .git — its contents are irrelevant
                    // and can contain thousands of object files.
                    walker.skip_current_dir();
                }
            }
        }
    }
}
