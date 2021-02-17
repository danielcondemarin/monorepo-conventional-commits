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
    repo_root: PathBuf,
    packages_globset: GlobSet,
}

impl Monorepo for LernaMonorepo {
    fn new(repo_root: PathBuf) -> Option<Box<dyn Monorepo>> {
        let mut globset = GlobSetBuilder::new();

        if let Some(config) = LernaMonorepo::parse_lerna_config(&repo_root) {
            for pattern in config.packages.iter() {
                globset.add(Glob::new(pattern).expect("invalid glob found in lerna.json packages"));
            }

            if let Ok(set) = globset.build() {
                let monorepo = LernaMonorepo {
                    repo_root,
                    packages_globset: set,
                };
                return Some(Box::new(monorepo));
            }
        }

        None
    }

    fn get_commit_scopes(&self, staged_changes: Vec<String>) -> Vec<String> {
        let mut packages_changed = HashMap::new();

        for path in staged_changes {
            log::info!("found staged entry {:#?}", path,);

            let package_name = self.get_package_name_for_file(&path);

            if let Some(name) = package_name {
                if let Some(name_str) = name.to_str() {
                    log::info!("got package name {}", name_str);
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
    fn parse_lerna_config(repo_path: &PathBuf) -> Option<LernaMonorepoConfig> {
        let lerna_config_path = repo_path.join("lerna.json");

        if let Ok(mut lerna_config_file) = File::open(lerna_config_path) {
            let mut contents = String::new();

            if let Ok(_) = lerna_config_file.read_to_string(&mut contents) {
                if let Ok(config) = serde_json::from_str::<LernaMonorepoConfig>(contents.as_str()) {
                    return Some(config);
                }
            }
        }

        None
    }

    fn get_package_name_for_file(&self, entry: &str) -> Option<OsString> {
        let abs_path = self.repo_root.join(entry);
        let mut ancestors = abs_path.ancestors();

        while let Some(dir) = ancestors.next() {
            if dir.eq(self.repo_root.as_path()) {
                return None;
            }

            let package_json_path = Path::new(self.repo_root.as_path())
                .join(dir)
                .join("package.json");

            if package_json_path.exists() {
                if self.packages_globset.is_match(entry) {
                    let package_name = dir.file_name().map(ToOwned::to_owned);
                    return package_name;
                }
            }
        }

        None
    }
}
