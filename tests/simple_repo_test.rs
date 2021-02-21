use nvim_conventional_commits::{CommitScopeFinder, DefaultScopeFinder};
mod common;
use common::TestOptionsBuilder;

#[test]
fn it_returns_no_scopes() {
    let options = TestOptionsBuilder::new("tests/fixtures/simple-repo").build();

    let cch = DefaultScopeFinder::new(&options.git_repo);
    let scopes = cch.get_commit_scopes();

    let empty_vec: Vec<String> = Vec::new();
    assert_eq!(scopes, empty_vec)
}
