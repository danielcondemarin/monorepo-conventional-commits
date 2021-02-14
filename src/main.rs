extern crate neovim_lib;

use logging::Logger;
use neovim_lib::{Neovim, NeovimApi, Session};
use nvim_conventional_commits::ConventionalCommitsHint;

mod logging;

struct EventHandler<'a> {
    nvim: Neovim,
    conventional_commits: ConventionalCommitsHint<'a>,
}

impl<'a> EventHandler<'a> {
    fn new() -> EventHandler<'a> {
        Logger::new().init().expect("failed to initialize logger");

        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        EventHandler {
            nvim,
            conventional_commits: ConventionalCommitsHint::new(
                "/Users/daniel/workspace/voice-common",
                None,
            ),
        }
    }

    fn recv(&mut self) {
        let rx = self.nvim.session.start_event_loop_channel();

        for (event, values) in rx {
            log::info!("event {:#?}. values {:#?}", event, values);

            self.nvim
                .command(&format!(
                    "echo \"{}\"",
                    self.conventional_commits.get_suggested_commit(),
                ))
                .unwrap()
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
