use git2::{Status, Statuses};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Deserialize;
use std::{collections::HashMap, ffi::OsString, fs::File, path::Path};
use std::{io::Read, path::PathBuf};

use crate::Monorepo;

#[derive(Deserialize)]
pub struct LernaMonorepoConfig {
    packages: Vec<String>,
}

pub struct LernaMonorepo {
    repo_path: PathBuf,
    packages_globset: GlobSet,
}

impl Monorepo for LernaMonorepo {
    fn new(repo_path: PathBuf) -> Option<Box<dyn Monorepo>> {
        let lerna_config_path = repo_path.join("lerna.json");

        if let Ok(mut lerna_config_file) = File::open(lerna_config_path) {
            let mut contents = String::new();

            if let Ok(_) = lerna_config_file.read_to_string(&mut contents) {
                if let Ok(config) = serde_json::from_str::<LernaMonorepoConfig>(contents.as_str()) {
                    let mut globset = GlobSetBuilder::new();
                    for pattern in config.packages.iter() {
                        globset.add(
                            Glob::new(pattern).expect("invalid glob found in lerna.json packages"),
                        );
                    }

                    if let Ok(set) = globset.build() {
                        let monorepo = LernaMonorepo {
                            repo_path,
                            packages_globset: set,
                        };
                        return Some(Box::new(monorepo));
                    }
                }
            }
        }

        None
    }

    fn get_commit_scopes(&self, statuses: Statuses) -> Vec<String> {
        let mut packages_changed = HashMap::new();

        for entry in statuses.iter() {
            let status = entry.status();
            // TODO: refactor and test only index files are checked
            if !status.contains(Status::INDEX_NEW)
                && !status.contains(Status::INDEX_MODIFIED)
                && !status.contains(Status::INDEX_DELETED)
                && !status.contains(Status::INDEX_RENAMED)
                && !status.contains(Status::INDEX_TYPECHANGE)
            {
                continue;
            }

            let package_name = self.get_package_name_for_file(entry.path().unwrap());

            if let Some(name) = package_name {
                if let Some(name_str) = name.to_str() {
                    packages_changed.entry(name_str.to_owned()).or_insert(true);
                }
            }
        }

        let mut sorted_packages: Vec<String> =
            packages_changed.keys().into_iter().cloned().collect();

        sorted_packages.sort();
        sorted_packages
    }
}

impl LernaMonorepo {
    fn get_package_name_for_file(&self, entry: &str) -> Option<OsString> {
        let abs_path = self.repo_path.join(entry);
        let mut ancestors = abs_path.ancestors();

        while let Some(parent) = ancestors.next() {
            if parent.eq(self.repo_path.as_path()) {
                return None;
            }

            let package_json_path = Path::new(self.repo_path.as_path())
                .join(parent)
                .join("package.json");

            if package_json_path.exists() {
                if self.packages_globset.is_match(entry) {
                    let package_name = parent.file_name().map(ToOwned::to_owned);
                    return package_name;
                }
            }
        }

        None
    }
}
