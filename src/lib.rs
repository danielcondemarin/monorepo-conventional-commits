use git2::{Repository, RepositoryOpenFlags, StatusOptions};
use std::{collections::HashMap, ffi::OsString, path::Path};

pub struct ConventionalCommitsHint<'a> {
    pub repo_path: &'a str,
}

impl<'a> ConventionalCommitsHint<'a> {
    fn get_package_name_for_file(&self, entry: &'a str) -> Option<OsString> {
        let abs_path = Path::new(&self.repo_path).join(entry);

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

        if packages_changed.len() > 0 {
            // TODO: use into_keys once api becomes stable, see https://github.com/rust-lang/rust/issues/75294
            let mut vec = packages_changed.keys().cloned().collect::<Vec<String>>();
            vec.sort();

            return format!("chore({}): commit message", vec.join(","));
        }

        "chore: commit message".to_string()
    }
}
