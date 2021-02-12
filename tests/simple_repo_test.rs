use nvim_conventional_commits::COMMIT_TYPES;

mod common;

#[test]
fn it_returns_default_commit_message() {
    let commit_msg = common::sut("tests/fixtures/simple-repo", None, None);
    assert_eq!(commit_msg, "chore: commit message")
}

#[test]
fn it_returns_commit_message_type() {
    for (shorthand, commit_type) in COMMIT_TYPES.iter() {
        let commit_msg = common::sut("tests/fixtures/simple-repo", None, Some(shorthand));
        assert_eq!(commit_msg, format!("{}: commit message", commit_type))
    }
}
