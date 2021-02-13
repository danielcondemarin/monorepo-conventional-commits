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
        let commit_msg = common::sut(
            "tests/fixtures/lerna-monorepo",
            Some(vec!["packages/package1/*"]),
            Some(shorthand),
        );

        assert_eq!(
            commit_msg,
            format!("{}(package1): commit message", commit_type)
        )
    }
}

#[test]
fn it_respects_lerna_config() {
    let commit_msg = common::sut(
        "tests/fixtures/complex-lerna-monorepo",
        Some(vec!["apps/app1", "libs/lib1"]),
        None,
    );
    assert_eq!(commit_msg, "chore(app1,lib1): commit message")
}

#[test]
fn it_ignores_path_not_in_packages_config() {
    let commit_msg = common::sut(
        "tests/fixtures/complex-lerna-monorepo",
        Some(vec!["spikes/spike1/*"]),
        None,
    );
    assert_eq!(commit_msg, "chore: commit message")
}
