use nvim_conventional_commits::COMMIT_TYPES;

mod common;

#[test]
fn it_returns_commit_message_with_multiple_scopes() {
    let commit_msg = common::sut(
        "tests/fixtures/lerna-monorepo",
        Some(vec!["packages/package1/*", "packages/package2/*"]),
        None,
    );

    assert_eq!(commit_msg, "chore(package1,package2): commit message")
}

#[test]
fn it_returns_commit_message_with_one_scope() {
    let commit_msg = common::sut(
        "tests/fixtures/lerna-monorepo",
        Some(vec!["packages/package1/*"]),
        None,
    );

    assert_eq!(commit_msg, "chore(package1): commit message")
}

#[test]
fn it_returns_commit_message_type() {
    for (shorthand, commit_type) in COMMIT_TYPES.iter() {
        let commit_msg = common::sut("tests/fixtures/simple-repo", None, Some(shorthand));
        assert_eq!(commit_msg, format!("{}: commit message", commit_type))
    }
}
