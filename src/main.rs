mod app;
mod event;
mod ui;

use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;
use event::{Event, EventHandler};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let events = EventHandler::new(250);

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        match events.next()? {
            Event::Key(key) => {
                event::handle_key(&mut app, key);
            }
            Event::Tick => {}
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
