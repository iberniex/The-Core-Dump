/// application
pub mod app;

/// Terminal Event Handler.
pub mod event_t;

/// Widget renderer
pub mod ui;

/// Terminal User Interface.
pub mod tui;

/// Application updater
pub mod update;

use app::App;
use event_t::{Event, EventHandler};

use color_eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::Tui;

/// Common Patterns found in creating ratatui apps
/// 1. Initialize the terminal
/// 2. Run the application in a loop until the user exits the app
/// 3. Restore the terminal back to its original state.
fn main() -> Result<()> {
    // `ratatui::run` calls the initializer and the restore methods
    // and then creates and run the App.
    // Create the application
    let mut app = App::new();

    // Initialize the terminal user Interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend);
    let events = EventHandler::new(250);

    let mut tui = Tui::new(terminal?, events);
    tui.enter()?;

    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        //Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update::update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
