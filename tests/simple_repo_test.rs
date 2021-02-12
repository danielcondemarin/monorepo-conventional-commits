use nvim_conventional_commits::ConventionalCommitsHint;

mod common;

fn sut<'a>(fixture: &'a str, index_paths: Option<Vec<&str>>) -> String {
    let (td, _) = common::create_repo_from_fixture(fixture, index_paths);
    let repo_path = td.path().to_str().unwrap();
    let cch = ConventionalCommitsHint { repo_path };
    let suggested_commit = cch.get_suggested_commit();
    suggested_commit.to_string()
}

#[test]
fn lerna_it_returns_commit_message_with_one_scope() {
    let commit_msg = sut(
        "tests/fixtures/lerna-monorepo",
        Some(vec!["packages/package1/*"]),
    );

    assert_eq!(commit_msg, "chore(package1): commit message")
}

#[test]
fn it_returns_default_commit_message() {
    let commit_msg = sut("tests/fixtures/simple-repo", None);
    assert_eq!(commit_msg, "chore: commit message")
}
