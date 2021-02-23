use nvim_conventional_commits::{
    logger::Logger, prepare_commit_msg::PrepareCommitMessage, DefaultScopeFinder,
};
use std::{env, path::Path};

fn main() {
    let cwd = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    let commit_msg_file = args.get(1).unwrap();

    Logger::new().init().expect("failed to initialize logger");

    let prepare_commit_msg = PrepareCommitMessage {
        commit_msg_file: Path::new(commit_msg_file),
        commit_source: args.get(2).map(|cs| cs.as_str()),
        commit_msg_generator: &DefaultScopeFinder::new(&cwd),
    };

    prepare_commit_msg.update_commit().unwrap();
}
