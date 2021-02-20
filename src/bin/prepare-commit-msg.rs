use nvim_conventional_commits::ConventionalCommitsHint;
use std::{
    env,
    io::{Seek, SeekFrom, Write},
};

fn main() {
    let cwd = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();

    let commit_msg_file_path = args
        .get(1)
        .expect("expected commit msg file as first argument");

    println!("found commit message file {}", commit_msg_file_path);

    if let Some(commit_source) = args.get(2) {
        println!("found commit source {}", commit_source);
    }

    let conventional_commits = ConventionalCommitsHint::new(&cwd, Some("c"));

    let mut commit_msg_file = std::fs::OpenOptions::new()
        .append(true)
        .open(commit_msg_file_path)
        .expect("failed to open git commit msg file");

    let suggested_commit = conventional_commits.get_suggested_commit();

    println!("suggested commit {}", suggested_commit);

    commit_msg_file
        .seek(SeekFrom::Start(0))
        .expect("failed to update commit message");

    commit_msg_file
        .write_all(suggested_commit.as_bytes())
        .expect("failed to update commit message");

    println!("made it here");
}
