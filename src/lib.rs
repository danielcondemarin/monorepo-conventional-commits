use git2::{Repository, RepositoryOpenFlags, StatusOptions};
use std::{collections::HashMap, ffi::OsString, path::Path};

pub static COMMIT_TYPES: [(&'static str, &'static str); 10] = [
    ("b", "build"),
    ("ci", "ci"),
    ("c", "chore"),
    ("d", "docs"),
    ("f", "feat"),
    ("p", "perf"),
    ("r", "refactor"),
    ("rev", "revert"),
    ("s", "style"),
    ("t", "test"),
];

pub struct ConventionalCommitsHint<'a> {
    repo_path: &'a Path,
    commit_type_hint: Option<&'a str>,
    commit_types: HashMap<&'a str, &'a str>,
}

impl<'a> ConventionalCommitsHint<'a> {
    pub fn new(
        repo_path_str: &'a str,
        commit_type_hint: Option<&'a str>,
    ) -> ConventionalCommitsHint<'a> {
        ConventionalCommitsHint {
            repo_path: Path::new(repo_path_str),
            commit_type_hint,
            commit_types: COMMIT_TYPES.iter().cloned().collect(),
        }
    }

    pub fn get_suggested_commit(&self) -> String {
        let repo = Repository::open_ext(
            Path::new(self.repo_path),
            RepositoryOpenFlags::NO_SEARCH,
            Vec::<String>::new(),
        )
        .unwrap();

        let mut status_opts = StatusOptions::new();
        let mut packages_changed = HashMap::new();

        for entry in repo.statuses(Some(&mut status_opts)).unwrap().iter() {
            let package_name = self.get_package_name_for_file(entry.path().unwrap());

            if let Some(name) = package_name {
                if let Some(name_str) = name.to_str() {
                    packages_changed.entry(name_str.to_owned()).or_insert(true);
                }
            }
        }

        let commit_type_hint = self
            .commit_type_hint
            .map_or("chore", |ch| self.commit_types[ch]);

        if packages_changed.len() > 0 {
            // TODO: use into_keys once api becomes stable, see https://github.com/rust-lang/rust/issues/75294
            let mut vec = packages_changed.keys().cloned().collect::<Vec<String>>();
            vec.sort();

            return format!("chore({}): commit message", vec.join(","));
        }

        format!("{}: commit message", commit_type_hint)
    }

    fn get_package_name_for_file(&self, entry: &'a str) -> Option<OsString> {
        let abs_path = self.repo_path.join(entry);
        let mut ancestors = abs_path.ancestors();

        while let Some(parent) = ancestors.next() {
            if parent.eq(Path::new("packages")) || parent.eq(Path::new(self.repo_path)) {
                return None;
            }

            let package_json_path = Path::new(self.repo_path).join(parent).join("package.json");

            if package_json_path.exists() {
                let package_name = parent.file_name().map(ToOwned::to_owned);
                return package_name;
            }
        }

        None
    }
}
