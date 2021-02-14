use git2::{Repository, RepositoryOpenFlags, StatusOptions, Statuses};
use lerna::LernaMonorepo;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

mod lerna;

pub static COMMIT_TYPES: [(&'static str, &'static str); 11] = [
    ("b", "build"),
    ("ci", "ci"),
    ("c", "chore"),
    ("d", "docs"),
    ("f", "feat"),
    ("fi", "fix"),
    ("p", "perf"),
    ("r", "refactor"),
    ("rev", "revert"),
    ("s", "style"),
    ("t", "test"),
];

pub trait Monorepo {
    fn new(repo_path: PathBuf) -> Option<Box<dyn Monorepo>>
    where
        Self: Sized;
    fn get_commit_scopes(&self, statuses: Statuses) -> Vec<String>;
}

pub struct ConventionalCommitsHint<'a> {
    repo_path: &'a Path,
    commit_type_hint: Option<&'a str>,
    commit_types: HashMap<&'a str, &'a str>,
    monorepo: Option<Box<dyn Monorepo>>,
}

impl<'a> ConventionalCommitsHint<'a> {
    pub fn new(
        repo_path_str: &'a str,
        commit_type_hint: Option<&'a str>,
    ) -> ConventionalCommitsHint<'a> {
        let repo_path = Path::new(repo_path_str);

        ConventionalCommitsHint {
            repo_path,
            commit_type_hint,
            commit_types: COMMIT_TYPES.iter().cloned().collect(),
            monorepo: LernaMonorepo::new(repo_path.to_path_buf()),
        }
    }

    pub fn get_suggested_commit(&self) -> String {
        let repo = Repository::open_ext(
            self.repo_path,
            RepositoryOpenFlags::CROSS_FS,
            Vec::<String>::new(),
        )
        .expect(
            format!(
                "failed to load git repo from path given {}",
                self.repo_path.to_string_lossy()
            )
            .as_str(),
        );

        let mut status_opts = StatusOptions::new();
        let statuses = repo.statuses(Some(&mut status_opts)).unwrap();

        let commit_type = self
            .commit_type_hint
            .map_or("chore", |ch| self.commit_types[ch]);

        if let Some(monorepo) = &self.monorepo {
            let scopes = monorepo.get_commit_scopes(statuses).join(",");

            if scopes.len() > 0 {
                return format!("{}({}): commit message", commit_type, scopes);
            }
        }

        return format!("{}: commit message", commit_type);
    }
}
