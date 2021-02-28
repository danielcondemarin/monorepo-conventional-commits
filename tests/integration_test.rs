use nvim_conventional_commits::{CommitScopeFinder, DefaultScopeFinder};
mod common;
use common::TestOptionsBuilder;

use test_utilities::test_each;

struct TestCase<'a> {
    description: &'a str,
    fixture: &'a str,
    staged_paths: Vec<&'a str>,
    expected_scopes: Vec<&'a str>,
}

#[test]
fn basic_tests() {
    let test_cases = vec![
        TestCase {
            description: "it returns no scopes",
            fixture: "tests/fixtures/simple-repo",
            staged_paths: vec!["packages/package1/*", "packages/package2/*"],
            expected_scopes: vec![],
        },
        TestCase {
            description: "it returns commit message with multiple scopes",
            fixture: "tests/fixtures/lerna-monorepo",
            staged_paths: vec!["packages/package1/*", "packages/package2/*"],
            expected_scopes: vec!["package1", "package2"],
        },
        TestCase {
            description: "it returns commit message with one scope",
            fixture: "tests/fixtures/lerna-monorepo",
            staged_paths: vec!["packages/package1/*"],
            expected_scopes: vec!["package1"],
        },
        TestCase {
            description: "it respects lerna config",
            fixture: "tests/fixtures/complex-lerna-monorepo",
            staged_paths: vec!["apps/app1", "libs/lib1"],
            expected_scopes: vec!["app1", "lib1"],
        },
        TestCase {
            description: "it ignores path not in lerna packages config",
            fixture: "tests/fixtures/complex-lerna-monorepo",
            staged_paths: vec!["spikes/spike1/*"],
            expected_scopes: vec![],
        },
        TestCase {
            description: "it uses package.json name for the scope (not directory name)",
            fixture: "tests/fixtures/complex-lerna-monorepo",
            staged_paths: vec!["apps/app3"],
            expected_scopes: vec!["app3-x"],
        },
    ];

    test_each(test_cases, |tc| {
        println!("Running test: {}", tc.description);

        let options = TestOptionsBuilder::new(tc.fixture)
            .with_staged_paths(tc.staged_paths)
            .build();

        let cch = DefaultScopeFinder::new(&options.git_repo);
        let scopes = cch.get_commit_scopes();

        assert_eq!(scopes, tc.expected_scopes);
    })
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
