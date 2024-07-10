use crate::prelude::*;

use crossterm::{execute, event, terminal};
use ratatui::{Terminal, backend::CrosstermBackend, layout};

/// The Terminal type using Crossterm as backend
pub type Console = Terminal<CrosstermBackend<std::io::Stderr>>;

/// Initialize the panic hook to restore the terminal state on panic, avoiding a broken terminal
pub fn panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        execute!(std::io::stderr(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

/// Enter raw mode and return the terminal
pub fn enter_raw_mode() -> Result<Console> {
    terminal::enable_raw_mode()?;
    execute!(std::io::stderr(), terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    terminal.clear()?;
    Ok(terminal)
}

/// Exit raw mode
pub fn exit_raw_mode() -> Result<()> {
    execute!(std::io::stderr(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Render the terminal UI
pub fn render(ctx: &mut super::Context) -> Result<()> {
    use super::layout;

    while ctx.is_running() {
        ctx.terminal.draw(layout::Main::render)?;

        update(ctx)?;
    }
    Ok(())
}

// Check for events like key presses and mouse clicks. Return true if wants to exit // FIXME: Handling the exit using some kind of event/stream
fn update(ctx: &mut super::Context) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(ctx.refresh_rate))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char('q'|'Q') => ctx.exit(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}