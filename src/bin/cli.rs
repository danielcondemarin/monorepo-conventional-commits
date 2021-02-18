use std::env;

use nvim_conventional_commits::ConventionalCommitsHint;

fn main() {
    let cwd = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    let commit_type = args.get(1).map(|ct| ct.as_str());
    let conventional_commits = ConventionalCommitsHint::new(&cwd, commit_type);

    println!("{}", conventional_commits.get_suggested_commit());
}
