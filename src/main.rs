extern crate neovim_lib;

use std::path::Path;

use logger::Logger;
use neovim_lib::{Neovim, NeovimApi, Session};
use nvim_conventional_commits::ConventionalCommitsHint;

mod logger;

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

            log::info!("received values {:#?}\n", values);

            let conventional_commits = ConventionalCommitsHint::new(Path::new(repo_path), None);

            if let Err(error) = self.nvim.command(&format!(
                "normal i {}",
                conventional_commits.get_suggested_commit(),
            )) {
                log::error!("running command resulted in error {}", error);
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
