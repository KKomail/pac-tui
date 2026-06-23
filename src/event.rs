use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, Tab};

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate_ms);

        thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    if let Ok(CrosstermEvent::Key(key)) = event::read() {
                        let _ = tx.send(Event::Key(key));
                    }
                } else {
                    let _ = tx.send(Event::Tick);
                }
            }
        });

        Self { rx }
    }

    pub fn next(&self) -> std::io::Result<Event> {
        self.rx
            .recv()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

// Returns true if the app should quit.
pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if app.searching {
        return handle_search_key(app, key);
    }

    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            true
        }
        KeyCode::Tab => {
            app.current_tab = app.current_tab.next();
            app.selected_index = 0;
            false
        }
        KeyCode::BackTab => {
            app.current_tab = app.current_tab.prev();
            app.selected_index = 0;
            false
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_item();
            false
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.prev_item();
            false
        }
        KeyCode::Char('g') => {
            app.first_item();
            false
        }
        KeyCode::Char('G') => {
            app.last_item();
            false
        }
        KeyCode::Enter => {
            app.toggle_details();
            false
        }
        KeyCode::Char('/') if app.current_tab == Tab::Packages => {
            app.open_search();
            false
        }
        KeyCode::Esc => {
            if !app.search_query.is_empty() {
                app.close_search();
            } else {
                app.close_details();
            }
            false
        }
        _ => false,
    }
}

fn handle_search_key(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            app.close_search();
            false
        }
        KeyCode::Enter => {
            app.confirm_search();
            false
        }
        KeyCode::Backspace => {
            app.search_backspace();
            false
        }
        KeyCode::Char(c) => {
            app.search_input(c);
            false
        }
        _ => false,
    }
}
