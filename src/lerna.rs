use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap, fs::File, path::Path};
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
                let glob = GlobBuilder::new(pattern)
                    .literal_separator(true)
                    .build()
                    .expect("invalid glob found in lerna.json packages");
                globset.add(glob);
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
                log::info!("got package name {}", name);
                packages_changed.entry(name).or_insert(true);
            }
        }

        let mut sorted_packages: Vec<String> =
            packages_changed.keys().into_iter().cloned().collect();

        sorted_packages.sort();
        sorted_packages
    }
}

#[derive(Serialize, Deserialize)]
struct PackageJSON {
    name: String,
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

    fn get_package_name_for_file(&self, entry: &str) -> Option<String> {
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
                if let Ok(package_json_file) = File::open(&package_json_path) {
                    let package_json_result: Result<PackageJSON> =
                        serde_json::from_reader(package_json_file);

                    if let Ok(package_json) = package_json_result {
                        let dir_relative = dir.strip_prefix(&self.repo_root).unwrap();

                        if self.packages_globset.is_match(dir_relative) {
                            let name = package_json.name;

                            if name.starts_with("@") {
                                let name_parts: Vec<&str> = name.split("/").collect();
                                let actual_name = name_parts.get(1).map(|s| s.to_string());
                                return actual_name;
                            }

                            return Some(name);
                        }
                    }
                }
            }
        }

        None
    }
}
