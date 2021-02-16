use nvim_conventional_commits::{ConventionalCommitsHint, COMMIT_TYPES};
mod common;
use common::TestOptionsBuilder;

#[test]
fn it_returns_default_commit_message() {
    let options = TestOptionsBuilder::new("tests/fixtures/simple-repo").build();

    let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
    let commit_msg = cch.get_suggested_commit();

    assert_eq!(commit_msg, "chore: commit message")
}

#[test]
fn it_returns_commit_message_type() {
    for (shorthand, commit_type) in COMMIT_TYPES.iter() {
        let options = TestOptionsBuilder::new("tests/fixtures/simple-repo")
            .with_shorthand(shorthand)
            .build();

        let cch = ConventionalCommitsHint::new(&options.git_repo, options.shorthand);
        let commit_msg = cch.get_suggested_commit();

        assert_eq!(commit_msg, format!("{}: commit message", commit_type))
    }
}
