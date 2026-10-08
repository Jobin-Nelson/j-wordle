use crate::{
    Result,
    constants::{ATTEMPTS, N_LETTERS, TRIES},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,

    pub attempts: [char; ATTEMPTS],
    step: usize,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Char(x)) if self.step < ATTEMPTS => {
                self.attempts[self.step] = x;
                self.step = (self.step + 1).min(ATTEMPTS);
            }
            (_, KeyCode::Backspace) => {
                self.step = self.step.saturating_sub(1);
                self.attempts[self.step] = Default::default();
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_attempts() -> Result<()> {
        // -- Setup & Fixtures
        let mut app = App::new();
        let range = 'a'..'e';

        // -- Exec
        range
            .clone()
            .map(|c| KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty()))
            .for_each(|k| app.on_key_event(k));
        let mut expected: [char; N_LETTERS * TRIES] = Default::default();
        for (i, c) in range.enumerate() {
            expected[i] = c;
        }

        // -- Check
        assert_eq!(app.attempts, expected);
        // println!(
        //     "{:?}",
        //     app.attempts.chunks_exact(N_LETTERS).collect::<Vec<_>>()
        // );
        Ok(())
    }
}
