use std::env;

use nvim_conventional_commits::ConventionalCommitsHint;

fn main() {
    let cwd = env::current_dir().unwrap();
    let conventional_commits = ConventionalCommitsHint::new(&cwd, None);
    println!("{}", conventional_commits.get_suggested_commit());
}
