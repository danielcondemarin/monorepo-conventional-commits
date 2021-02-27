use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::CommitScopeFinder;

pub static COMMIT_TYPES: [&'static str; 11] = [
    "build", "ci", "chore", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test",
];

pub struct PrepareCommitMessage<'a> {
    pub commit_msg_file: &'a Path,
    pub commit_source: Option<&'a str>,
    pub commit_msg_generator: &'a dyn CommitScopeFinder,
}

impl<'a> PrepareCommitMessage<'a> {
    pub fn update_commit(&self) -> std::io::Result<()> {
        let scopes = self.commit_msg_generator.get_commit_scopes();

        if scopes.len() == 0 {
            return Ok(());
        }

        let mut commit_msg_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.commit_msg_file)?;

        let mut original_commit_msg = String::new();
        commit_msg_file.read_to_string(&mut original_commit_msg)?;

        let mut new_commit_msg: Option<String> = None;

        match self.commit_source {
            Some("message") => {
                new_commit_msg = self.handle_message_commit_source(&original_commit_msg, scopes)
            }
            Some(_) => {}
            None => new_commit_msg = self.handle_default(&original_commit_msg, scopes),
        }

        if let Some(msg) = new_commit_msg {
            commit_msg_file.seek(SeekFrom::Start(0))?;
            commit_msg_file.write_all(msg.as_bytes())?;
        }

        Ok(())
    }

    fn handle_default(&self, commit_msg: &'a str, scopes: Vec<String>) -> Option<String> {
        Some(format!("chore({}):\n{}", scopes.join(","), commit_msg))
    }

    fn handle_message_commit_source(
        &self,
        commit_msg: &'a str,
        scopes: Vec<String>,
    ) -> Option<String> {
        if let Some(first_line) = commit_msg.lines().next() {
            if let Some(commit_type) = COMMIT_TYPES
                .iter()
                .find(|ct| first_line.starts_with(&format!("{}:", ct)))
            {
                return Some(commit_msg.replace(
                    &format!("{}:", commit_type),
                    &format!("{}({}):", commit_type, scopes.join(",")),
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use std::{
        io::SeekFrom,
        panic::{catch_unwind, AssertUnwindSafe, UnwindSafe},
    };
    use tempfile::NamedTempFile;

    fn test_each<'a, T, A>(test_cases: Vec<A>, test: T)
    where
        T: Clone + Fn(A) -> () + UnwindSafe,
        A: UnwindSafe,
    {
        for args in test_cases {
            let test_clone = AssertUnwindSafe(test.clone());
            let result = catch_unwind(|| test_clone(args));
            assert!(result.is_ok())
        }
    }

    struct MockCommitScopeFinder {
        scopes: Vec<String>,
    }

    impl CommitScopeFinder for MockCommitScopeFinder {
        fn get_commit_scopes(&self) -> Vec<String> {
            self.scopes.clone()
        }
    }

    fn create_tmp_git_commit_file(contents: &str) -> NamedTempFile {
        let mut tmp_file = NamedTempFile::new().unwrap();
        tmp_file.write_all(contents.as_bytes()).unwrap();
        tmp_file
    }

    fn read_git_commit_file(mut file: NamedTempFile) -> String {
        let mut commit_msg_contents = String::new();
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_to_string(&mut commit_msg_contents).unwrap();

        commit_msg_contents
    }

    #[test]
    fn test_scenarios() {
        struct TestCase<'a> {
            description: &'a str,
            commit_msg_contents: &'a str,
            commit_source: Option<&'a str>,
            scopes: Vec<&'a str>,
            expected_commit: &'a str,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                description: "does not do anything when commit source = \"squash\"",
                commit_source: Some("squash"),
                scopes: vec!["app1"],
                commit_msg_contents: "some commit message",
                expected_commit: "some commit message",
            },
            TestCase {
                description: "does not do anything when commit source = \"commit\"",
                commit_source: Some("commit"),
                scopes: vec!["app1"],
                commit_msg_contents: "some commit message",
                expected_commit: "some commit message",
            },
            TestCase {
                description: "does not do anything when commit source = \"squash\"",
                commit_source: Some("merge"),
                scopes: vec!["app1"],
                commit_msg_contents: "some commit message",
                expected_commit: "some commit message",
            },
            TestCase {
                description: "does not do anything when commit source = \"anything_else\"",
                commit_source: Some("anything_else"),
                scopes: vec!["app1"],
                commit_msg_contents: "some commit message",
                expected_commit: "some commit message",
            },
            TestCase {
                description: "does not do anything when no scopes are affected",
                commit_source: None,
                scopes: vec![],
                commit_msg_contents: "some commit message",
                expected_commit: "some commit message",
            },
            TestCase {
                description: "reuses commit type from user input",
                commit_source: Some("message"),
                scopes: vec!["package"],
                commit_msg_contents: "docs: commit message with type specified already",
                expected_commit: "docs(package): commit message with type specified already",
            },
            TestCase {
                description:
                    "adds generated commit message to the beginning of commit message file",
                commit_source: None,
                scopes: vec!["package"],
                commit_msg_contents: "initial contents of commit message\nwith multiple lines",
                expected_commit:
                    "chore(package):\ninitial contents of commit message\nwith multiple lines",
            },
        ];

        test_each(test_cases, |tc| {
            println!("Running test: {}", tc.description);

            let tmp_file = create_tmp_git_commit_file(tc.commit_msg_contents);

            let pcm = PrepareCommitMessage {
                commit_msg_file: tmp_file.path(),
                commit_source: tc.commit_source,
                commit_msg_generator: &MockCommitScopeFinder {
                    scopes: tc.scopes.iter().map(|s| s.to_string()).collect(),
                },
            };

            pcm.update_commit().unwrap();

            assert_eq!(read_git_commit_file(tmp_file), tc.expected_commit);
        });
    }
}
