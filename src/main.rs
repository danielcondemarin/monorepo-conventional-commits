extern crate neovim_lib;

use logging::Logger;
use neovim_lib::{Neovim, NeovimApi, Session};
use nvim_conventional_commits::ConventionalCommitsHint;

mod logging;

struct EventHandler {
    nvim: Neovim,
}

impl EventHandler {
    fn new() -> EventHandler {
        Logger::new().init().expect("failed to initialize logger");

        EventHandler {
            nvim: Neovim::new(Session::new_parent().unwrap()),
        }
    }

    fn recv(&mut self) {
        let rx = self.nvim.session.start_event_loop_channel();

        for (_, values) in rx {
            let repo_path = values[0]
                .as_str()
                .expect("expected first argument in message to be repo url");

            let conventional_commits = ConventionalCommitsHint::new(repo_path, None);

            self.nvim
                .command(&format!(
                    "echo \"{}\"",
                    conventional_commits.get_suggested_commit(),
                ))
                .unwrap()
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
