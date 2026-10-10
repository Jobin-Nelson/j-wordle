use crate::{
    Result,
    constants::{ATTEMPTS, M_UNFILLED, M_WON, N_LETTERS},
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub mode: AppMode,

    pub attempts: [char; ATTEMPTS],         // tracks the guesses
    pub guess_types: [GuessType; ATTEMPTS], // tracks the guess state
    step: usize,                            // tracks the current position
    offset: usize,                          // limits the editable portion
    chosen_id: usize,
    corpus: Vec<String>,
    pub message: &'static str,
}

#[derive(Debug, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Running,
    ReloadApp,
    Quit,
}

#[derive(Debug, Default, PartialEq)]
pub enum GuessType {
    Green,
    Yellow,
    Gray,
    #[default]
    NotAttempted,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let corpus = vec![
            "bible".to_string(),
            "edify".to_string(),
            "mouse".to_string(),
        ];
        let chosen_id = rand::random_range(0..corpus.len());
        // let chosen_id = 0;
        Self {
            chosen_id,
            corpus,
            ..Default::default()
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    pub fn handle_crossterm_events(&mut self) -> Result<()> {
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
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (KeyModifiers::CONTROL, KeyCode::Char('r')) => self.mode = AppMode::ReloadApp,
            (_, KeyCode::Char(x)) if self.step < ATTEMPTS && !self.is_fully_guessed() => {
                self.attempts[self.step] = x;
                self.step = (self.step + 1).min(ATTEMPTS);
            }
            (_, KeyCode::Backspace) if self.step > self.offset => {
                self.step = self.step.saturating_sub(1);
                self.attempts[self.step] = Default::default();
            }
            (_, KeyCode::Enter) => {
                if self.is_fully_guessed() {
                    self.check_guess();
                } else {
                    // TODO: Display warning
                    self.message = M_UNFILLED;
                }
            }
            _ => {}
        }
    }

    fn is_fully_guessed(&self) -> bool {
        self.offset + N_LETTERS == self.step
    }

    fn check_guess(&mut self) {
        let chosen_word: Vec<_> = self.corpus[self.chosen_id].chars().collect();

        let start = self.offset;
        let mut char_used = [false; N_LETTERS];

        // Mark for greens
        for i in 0..N_LETTERS {
            let adjusted_id = start + i;
            let guess_char = self.attempts[adjusted_id];
            if guess_char.eq_ignore_ascii_case(&chosen_word[i]) {
                self.guess_types[adjusted_id] = GuessType::Green;
                char_used[i] = true;
            } else {
                self.guess_types[adjusted_id] = GuessType::Gray;
            }
        }
        if char_used.iter().all(|u| *u) {
            self.message = M_WON;
            return;
        }

        // Mark for yellow
        for i in 0..N_LETTERS {
            let adjusted_id = start + i;

            if self.guess_types[adjusted_id] == GuessType::Green {
                continue;
            }
            let guess_char = self.attempts[adjusted_id];

            for j in 0..N_LETTERS {
                if chosen_word[j] == guess_char && !char_used[j] {
                    self.guess_types[adjusted_id] = GuessType::Yellow;
                    char_used[j] = true;
                    break;
                }
            }
        }

        self.offset += N_LETTERS;
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.mode = AppMode::Quit;
    }
}

#[cfg(test)]
mod test {
    use super::GuessType::*;
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
        let mut expected: [char; ATTEMPTS] = Default::default();
        for (i, c) in range.enumerate() {
            expected[i] = c;
        }

        // -- Check
        assert_eq!(app.attempts, expected);
        Ok(())
    }

    #[test]
    fn test_check_guess() -> Result<()> {
        // -- Setup & Fixtures
        let mut app = App::new();
        app.corpus = vec![
            "bible".to_string(),
            "edify".to_string(),
            "mouse".to_string(),
        ];
        app.chosen_id = 0;

        let attempts = [
            ['b', 'e', 'a', 's', 't'], // 1 attempt
            ['b', 'e', 'a', 's', 'e'], // 2 attempt
            ['b', 'e', 'e', 's', 't'], // 3 attempt
        ];
        let mut expected: [GuessType; ATTEMPTS] = Default::default();
        let expected_guess_types = [
            [Green, Yellow, Gray, Gray, Gray], // 1 attempt
            [Green, Gray, Gray, Gray, Green],  // 2 attempt
            [Green, Yellow, Gray, Gray, Gray], // 3 attempt
        ];

        let mut start = 0;
        for (attempt, guess) in attempts.iter().zip(expected_guess_types) {
            // -- Exec
            for (i, c) in attempt.iter().enumerate() {
                app.attempts[start + i] = *c
            }
            for (i, g) in guess.into_iter().enumerate() {
                expected[start + i] = g
            }

            app.check_guess();

            // -- Check
            assert_eq!(app.guess_types, expected);

            start += N_LETTERS;
        }

        Ok(())
    }

    #[test]
    fn test_fully_guessed() -> Result<()> {
        // -- Setup & Fixtures
        let mut app = App::new();
        let attempts = [
            ['b', 'e', 'a', 's', 't'], // 1 attempt
            ['b', 'e', 'a', 's', 'e'], // 2 attempt
            ['b', 'e', 'e', 's', 't'], // 3 attempt
        ];

        for attempt in attempts {
            // -- Exec
            assert!(!app.is_fully_guessed());
            for k in attempt.map(|c| KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())) {
                assert!(!app.is_fully_guessed());
                app.on_key_event(k);
            }
            // -- Check
            assert!(app.is_fully_guessed());
            app.check_guess();
        }
        Ok(())
    }
}
