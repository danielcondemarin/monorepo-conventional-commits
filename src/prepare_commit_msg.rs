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
    use std::io::SeekFrom;
    use tempfile::NamedTempFile;

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
    fn ignores_unsupported_commit_sources() {
        let unsupported_commit_sources = vec!["template", "squash", "commit", "anything_else"];

        for src in unsupported_commit_sources {
            let tmp_file = create_tmp_git_commit_file("some commit message");

            let pcm = PrepareCommitMessage {
                commit_msg_file: tmp_file.path(),
                commit_source: Some(src),
                commit_msg_generator: &MockCommitScopeFinder {
                    scopes: vec!["app1".to_string()],
                },
            };

            pcm.update_commit().unwrap();

            assert_eq!(read_git_commit_file(tmp_file), "some commit message");
        }
    }

    #[test]
    fn doesnt_do_anything_when_no_scopes_affected() {
        let tmp_file = create_tmp_git_commit_file("some commit message");

        let pcm = PrepareCommitMessage {
            commit_msg_file: tmp_file.path(),
            commit_source: None,
            commit_msg_generator: &MockCommitScopeFinder { scopes: vec![] },
        };

        pcm.update_commit().unwrap();

        assert_eq!(read_git_commit_file(tmp_file), "some commit message");
    }

    #[test]
    fn reuses_commit_type_from_user_input() {
        let tmp_file =
            create_tmp_git_commit_file("docs: commit message with type specified already");

        let pcm = PrepareCommitMessage {
            commit_msg_file: tmp_file.path(),
            commit_source: Some("message"),
            commit_msg_generator: &MockCommitScopeFinder {
                scopes: vec!["package".to_string()],
            },
        };

        pcm.update_commit().unwrap();

        assert_eq!(
            read_git_commit_file(tmp_file),
            "docs(package): commit message with type specified already"
        );
    }

    #[test]
    fn prepends_generated_commit_message() {
        let tmp_file =
            create_tmp_git_commit_file("initial contents of commit message\nwith multiple lines");

        let pcm = PrepareCommitMessage {
            commit_msg_file: tmp_file.path(),
            commit_source: None,
            commit_msg_generator: &MockCommitScopeFinder {
                scopes: vec!["package".to_string()],
            },
        };

        pcm.update_commit().unwrap();

        assert_eq!(
            read_git_commit_file(tmp_file),
            "chore(package):\ninitial contents of commit message\nwith multiple lines"
        );
    }
}
