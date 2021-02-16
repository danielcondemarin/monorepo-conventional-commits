use nvim_conventional_commits::{ConventionalCommitsHint, COMMIT_TYPES};
mod common;
use common::TestOptionsBuilder;

#[test]
fn it_returns_commit_message_with_multiple_scopes() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*", "packages/package2/*"])
        .build();

    let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore(package1,package2): commit message")
}

#[test]
fn it_returns_commit_message_with_one_scope() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*"])
        .build();

    let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore(package1): commit message")
}

#[test]
fn it_returns_commit_message_type() {
    for (shorthand, commit_type) in COMMIT_TYPES.iter() {
        let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
            .with_staged_paths(vec!["packages/package1/*"])
            .with_shorthand(shorthand)
            .build();

        let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
        let commit_msg = cch.get_suggested_commit();

        assert_eq!(
            commit_msg,
            format!("{}(package1): commit message", commit_type)
        )
    }
}

#[test]
fn it_respects_lerna_config() {
    let options = TestOptionsBuilder::new("tests/fixtures/complex-lerna-monorepo")
        .with_staged_paths(vec!["apps/app1", "libs/lib1"])
        .build();

    let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore(app1,lib1): commit message")
}

#[test]
fn it_ignores_path_not_in_packages_config() {
    let options = TestOptionsBuilder::new("tests/fixtures/complex-lerna-monorepo")
        .with_staged_paths(vec!["spikes/spike1/*"])
        .build();

    let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore: commit message")
}

#[test]
fn it_handles_repo_subdirectory() {
    let options = TestOptionsBuilder::new("tests/fixtures/lerna-monorepo")
        .with_staged_paths(vec!["packages/package1/*"])
        .build();

    let cch = ConventionalCommitsHint::new(
        &options.git_repo.join("packages/package1"),
        options.shorthand,
    );
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore(package1): commit message")
}
