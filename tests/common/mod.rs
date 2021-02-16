use git2::{IndexAddOption, Repository, RepositoryInitOptions};
use path::{Path, PathBuf};
use std::{env, fs, path};
use tempfile::TempDir;
use walkdir::WalkDir;

pub struct TestOptions<'a> {
    fixture: PathBuf,
    staged_paths: Vec<&'a str>,
    pub git_repo: PathBuf,
    pub shorthand: Option<&'a str>,
}

pub struct TestOptionsBuilder<'a> {
    test_options: TestOptions<'a>,
}

impl<'a> TestOptionsBuilder<'a> {
    pub fn new(fixture: &'a str) -> Self {
        let td = TempDir::new().unwrap();

        TestOptionsBuilder {
            test_options: TestOptions {
                fixture: Path::new(fixture).to_owned(),
                staged_paths: vec!["*"],
                shorthand: Some("c"),
                git_repo: td.path().to_owned(),
            },
        }
    }

    pub fn with_staged_paths(mut self, staged_paths: Vec<&'a str>) -> Self {
        self.test_options.staged_paths = staged_paths;
        self
    }

    pub fn with_shorthand(mut self, shorthand: &'a str) -> Self {
        self.test_options.shorthand = Some(shorthand);
        self
    }

    pub fn build(self) -> TestOptions<'a> {
        create_repo_from_fixture(
            &self.test_options.fixture,
            &self.test_options.git_repo,
            &self.test_options.staged_paths,
        );

        self.test_options
    }
}

fn create_repo_from_fixture<'a>(fixture: &Path, dst: &Path, staged_paths: &Vec<&str>) {
    let repo = init_git_repo(dst);
    copy_fixture_to_tmp_repo(fixture, dst);
    add_working_dir_to_index(&repo, staged_paths);
}

fn add_working_dir_to_index(repo: &Repository, staged_paths: &Vec<&str>) {
    let mut index = repo.index().unwrap();
    let err = "failed to add fixture files to git index";

    index
        .add_all(staged_paths, IndexAddOption::DEFAULT, None)
        .expect(err);
    index.write().expect(err);
}

fn init_git_repo(temp_dir: &Path) -> Repository {
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");

    Repository::init_opts(temp_dir, &opts).expect("failed to initialise git repo")
}

fn copy_fixture_to_tmp_repo<'a>(fixture: &Path, repo: &Path) {
    let cargo_manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_path = Path::new(cargo_manifest.to_str().unwrap());
    let abs_fixture_path = cargo_manifest_path.join(fixture);

    for entry in WalkDir::new(&abs_fixture_path) {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.eq(&abs_fixture_path) {
            continue;
        }

        let src_relative_to_fixture = entry_path
            .strip_prefix::<&PathBuf>(&abs_fixture_path)
            .unwrap();

        let dst: PathBuf = [repo, src_relative_to_fixture].iter().collect();

        if entry_path.is_file() {
            fs::copy(entry_path, dst).unwrap();
        } else if entry_path.is_dir() {
            // println!("dst {:#?}", dst);
            fs::create_dir(dst).unwrap();
        }
    }
}
