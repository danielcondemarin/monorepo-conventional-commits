use nvim_conventional_commits::{CommitScopeFinder, DefaultScopeFinder};
mod common;
use common::TestOptionsBuilder;

#[test]
fn it_returns_commit_message_with_multiple_scopes() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*", "packages/package2/*"])
        .build();

    let cch = DefaultScopeFinder::new(&options.git_repo);
    let scopes = cch.get_commit_scopes();

    assert_eq!(scopes, vec!["package1", "package2"]);
}

#[test]
fn it_returns_commit_message_with_one_scope() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*"])
        .build();

    let cch = DefaultScopeFinder::new(&options.git_repo);
    let commit_msg = cch.get_commit_scopes();

    assert_eq!(commit_msg, vec!["package1"]);
}

#[test]
fn it_respects_lerna_config() {
    let options = TestOptionsBuilder::new("tests/fixtures/complex-lerna-monorepo")
        .with_staged_paths(vec!["apps/app1", "libs/lib1"])
        .build();

    let cch = DefaultScopeFinder::new(&options.git_repo);
    let commit_msg = cch.get_commit_scopes();

    assert_eq!(commit_msg, vec!["app1", "lib1"]);
}

#[test]
fn it_ignores_path_not_in_packages_config() {
    let options = TestOptionsBuilder::new("tests/fixtures/complex-lerna-monorepo")
        .with_staged_paths(vec!["spikes/spike1/*"])
        .build();

    let cch = DefaultScopeFinder::new(&options.git_repo);
    let scopes = cch.get_commit_scopes();

    let empty_vec: Vec<String> = Vec::new();
    assert_eq!(scopes, empty_vec);
}

#[test]
fn it_handles_repo_subdirectory() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*"])
        .build();

    let cch = DefaultScopeFinder::new(&options.git_repo.join("packages/package1"));
    let commit_msg = cch.get_commit_scopes();

    assert_eq!(commit_msg, vec!["package1"]);
}
