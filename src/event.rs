use std::time::Duration;

use crate::app::app_mod::{App, States};
use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
pub fn events(App: &mut App) -> Result<bool> {
    let state = App.get_state();
    let mut quit = false;
    match state {
        States::NORMAL(active_tab) => {
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        quit = true;
                    }
                }
            }
        }
    }
    Ok(quit)
}
