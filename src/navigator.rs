use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_git_repos(start_path: &str) -> Vec<PathBuf> {
  let mut git_dirs = Vec::new();
  for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
      let path = entry.path();
      if path.ends_with(".git") {
          git_dirs.push(path.to_path_buf());
      }
  }
  git_dirs
}
