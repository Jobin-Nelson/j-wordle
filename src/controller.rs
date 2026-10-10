use crate::{App, Result, app::AppMode};
use ratatui::DefaultTerminal;

/// Run the application's main loop.
pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = reload_app();
    loop {
        while app.mode == AppMode::Running {
            terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
            app.handle_crossterm_events()?;
        }
        match app.mode {
            AppMode::Running => todo!(),
            AppMode::ReloadApp => app = reload_app(),
            AppMode::Quit => break,
        }
    }
    Ok(())
}

pub fn reload_app() -> App {
    App::new()
}
