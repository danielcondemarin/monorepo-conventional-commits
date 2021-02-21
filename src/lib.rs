use git2::{Repository, RepositoryOpenFlags, Status, StatusOptions};
use lerna::LernaMonorepo;
use std::path::{Path, PathBuf};

mod lerna;

pub mod logger;
pub mod prepare_commit_msg;

pub trait Monorepo {
    fn new(repo_path: PathBuf) -> Option<Box<dyn Monorepo>>
    where
        Self: Sized;
    fn get_commit_scopes(&self, statuses: Vec<String>) -> Vec<String>;
}

pub trait CommitScopeFinder {
    fn get_commit_scopes(&self) -> Vec<String>;
}

pub struct DefaultScopeFinder {
    repo: Repository,
    monorepo: Option<Box<dyn Monorepo>>,
}

impl<'a> CommitScopeFinder for DefaultScopeFinder {
    fn get_commit_scopes(&self) -> Vec<String> {
        let mut status_opts = StatusOptions::new();

        let statuses = self.repo.statuses(Some(&mut status_opts)).unwrap();

        log::info!("has monorepo {}\n", &self.monorepo.is_some());

        // TODO: test only index files are considered to return scopes
        let index_statuses: [Status; 5] = [
            Status::INDEX_NEW,
            Status::INDEX_MODIFIED,
            Status::INDEX_DELETED,
            Status::INDEX_RENAMED,
            Status::INDEX_TYPECHANGE,
        ];

        let staged_changes: Vec<String> = statuses
            .iter()
            .filter(|entry| {
                index_statuses
                    .iter()
                    .any(|s| entry.status().contains(s.to_owned()))
            })
            .map(|entry| {
                return entry.path().unwrap().to_owned();
            })
            .collect();

        if let Some(monorepo) = &self.monorepo {
            return monorepo.get_commit_scopes(staged_changes);
        }

        return vec![];
    }
}

impl DefaultScopeFinder {
    pub fn new(repo_path_str: &Path) -> DefaultScopeFinder {
        let repo = Repository::open_ext(
            repo_path_str,
            RepositoryOpenFlags::CROSS_FS,
            Vec::<String>::new(),
        )
        .expect(&format!(
            "failed to load git repo from path given {}",
            repo_path_str.to_str().unwrap()
        ));

        let repo_root = repo.path().parent().unwrap().to_path_buf();
        let monorepo = LernaMonorepo::new(repo_root);

        DefaultScopeFinder { repo, monorepo }
    }
}
