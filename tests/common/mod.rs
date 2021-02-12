use git2::{IndexAddOption, Repository, RepositoryInitOptions};
use path::{Path, PathBuf};
use std::{env, fs, path};
use tempfile::TempDir;
use walkdir::WalkDir;

pub fn create_repo_from_fixture<'a>(
    fixture_path: &'a str,
    index_paths: Option<Vec<&str>>,
) -> (TempDir, Repository) {
    let td = TempDir::new().unwrap();

    let repo = init_git_repo(td.path());

    copy_fixture_files_to_temp_dir(Path::new(fixture_path), td.path());
    add_working_dir_to_index(&repo, index_paths);

    (td, repo)
}

fn add_working_dir_to_index(repo: &Repository, index_paths: Option<Vec<&str>>) {
    let mut index = repo.index().unwrap();
    let err = "failed to add fixture files to git index";

    let all_files = vec!["*"];

    index
        .add_all(
            index_paths.unwrap_or(all_files).iter(),
            IndexAddOption::DEFAULT,
            None,
        )
        .expect(err);
    index.write().expect(err);
}

fn init_git_repo(temp_dir_path: &Path) -> Repository {
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");
    Repository::init_opts(temp_dir_path, &opts).expect("failed to initialise git repo")
}

fn copy_fixture_files_to_temp_dir<'a>(fixture_path: &Path, temp_dir_path: &Path) {
    let cargo_manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_path = Path::new(cargo_manifest.to_str().unwrap());
    let abs_fixture_path = cargo_manifest_path.join(fixture_path);

    for entry in WalkDir::new(&abs_fixture_path) {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.eq(&abs_fixture_path) {
            continue;
        }

        let src_relative_to_fixture = entry_path
            .strip_prefix::<&PathBuf>(&abs_fixture_path)
            .unwrap();
        let dst: PathBuf = [temp_dir_path, src_relative_to_fixture].iter().collect();

        if entry_path.is_file() {
            fs::copy(entry_path, dst).unwrap();
        } else if entry_path.is_dir() {
            fs::create_dir(&dst).unwrap();
        }
    }
}
