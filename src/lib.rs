use git2::{Repository, RepositoryOpenFlags, StatusOptions};
use std::{ffi::OsString, path::Path};

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

            // println!("");
            // println!("parent {:#?}", parent);
            // println!("");

            let package_json_path = Path::new(self.repo_path).join(parent).join("package.json");

            if package_json_path.exists() {
                // println!("");
                // println!("found package json {:#?}", package_json_path);
                // println!("");

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

        for entry in repo.statuses(Some(&mut status_opts)).unwrap().iter() {
            let package_name = self.get_package_name_for_file(entry.path().unwrap());

            if let Some(ref name) = package_name {
                if let Some(ref name_str) = name.to_str() {
                    return format!("chore({}): commit message", name_str);
                }
            }
        }

        "chore: commit message".to_string()
    }
}
