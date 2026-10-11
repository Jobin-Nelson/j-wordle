use crate::{
    Result,
    constants::{M_LOST, M_UNFILLED, M_WON, N_GUESSES, N_LETTERS},
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub mode: AppMode,

    pub guesses: [char; N_GUESSES],          // tracks the guesses
    pub guess_types: [GuessType; N_GUESSES], // tracks the guess state
    guess_position: usize,                   // tracks the current position
    guessed_offset: usize,                   // limits the editable portion
    chosen_idx: usize,
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
            chosen_idx: chosen_id,
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
            (_, KeyCode::Char(x))
                if self.guess_position < N_GUESSES && !self.is_valid_attempt() =>
            {
                self.guesses[self.guess_position] = x;
                self.guess_position = (self.guess_position + 1).min(N_GUESSES);
            }
            (_, KeyCode::Backspace) if self.guess_position > self.guessed_offset => {
                self.guess_position = self.guess_position.saturating_sub(1);
                self.guesses[self.guess_position] = Default::default();
            }
            (_, KeyCode::Enter) => {
                if self.is_valid_attempt() {
                    self.validate_attempt();
                } else {
                    self.message = M_UNFILLED;
                }
            }
            _ => {}
        }
    }

    fn is_valid_attempt(&self) -> bool {
        self.guessed_offset + N_LETTERS == self.guess_position
    }

    fn validate_attempt(&mut self) {
        let chosen_word: Vec<_> = self.corpus[self.chosen_idx].chars().collect();

        let start = self.guessed_offset;
        let mut char_used = [false; N_LETTERS];

        // Mark for greens
        for i in 0..N_LETTERS {
            let adjusted_idx = start + i;
            let guess_char = self.guesses[adjusted_idx];
            if guess_char.eq_ignore_ascii_case(&chosen_word[i]) {
                self.guess_types[adjusted_idx] = GuessType::Green;
                char_used[i] = true;
            } else {
                self.guess_types[adjusted_idx] = GuessType::Gray;
            }
        }
        if char_used.iter().all(|u| *u) {
            self.message = M_WON;
            return;
        }

        if self.guess_position == N_GUESSES {
            self.message = M_LOST;
            return;
        }

        // Mark for yellow
        for i in 0..N_LETTERS {
            let adjusted_idx = start + i;

            if self.guess_types[adjusted_idx] == GuessType::Green {
                continue;
            }
            let guess_char = self.guesses[adjusted_idx];

            for j in 0..N_LETTERS {
                if chosen_word[j] == guess_char && !char_used[j] {
                    self.guess_types[adjusted_idx] = GuessType::Yellow;
                    char_used[j] = true;
                    break;
                }
            }
        }

        self.guessed_offset += N_LETTERS;
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
        let mut expected: [char; N_GUESSES] = Default::default();
        for (i, c) in range.enumerate() {
            expected[i] = c;
        }

        // -- Check
        assert_eq!(app.guesses, expected);
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
        app.chosen_idx = 0;

        let attempts = [
            ['b', 'e', 'a', 's', 't'], // 1 attempt
            ['b', 'e', 'a', 's', 'e'], // 2 attempt
            ['b', 'e', 'e', 's', 't'], // 3 attempt
        ];
        let mut expected: [GuessType; N_GUESSES] = Default::default();
        let expected_guess_types = [
            [Green, Yellow, Gray, Gray, Gray], // 1 attempt
            [Green, Gray, Gray, Gray, Green],  // 2 attempt
            [Green, Yellow, Gray, Gray, Gray], // 3 attempt
        ];

        let mut start = 0;
        for (attempt, guess) in attempts.iter().zip(expected_guess_types) {
            // -- Exec
            for (i, c) in attempt.iter().enumerate() {
                app.guesses[start + i] = *c
            }
            for (i, g) in guess.into_iter().enumerate() {
                expected[start + i] = g
            }

            app.validate_attempt();

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
            assert!(!app.is_valid_attempt());
            for k in attempt.map(|c| KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())) {
                assert!(!app.is_valid_attempt());
                app.on_key_event(k);
            }
            // -- Check
            assert!(app.is_valid_attempt());
            app.validate_attempt();
        }
        Ok(())
    }

    #[test]
    fn test_lost_messsages() -> Result<()> {
        // -- Setup & Fixtures
        let mut app = App::new();
        app.corpus = vec![
            "bible".to_string(),
            "edify".to_string(),
            "mouse".to_string(),
        ];
        app.chosen_idx = 0;
        let attempts = [
            ['b', 'e', 'b', 'l', 'e'], // 1 attempt
            ['b', 'e', 'a', 's', 'e'], // 2 attempt
            ['b', 'e', 'e', 's', 't'], // 3 attempt
            ['b', 'e', 'e', 's', 't'], // 4 attempt
            ['b', 'e', 'e', 's', 't'], // 5 attempt
        ];

        // -- Exec
        for attempt in attempts {
            assert!(!app.is_valid_attempt());
            for k in attempt.map(|c| KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())) {
                assert!(!app.is_valid_attempt());
                app.on_key_event(k);
            }
            app.validate_attempt();
        }

        // -- Check
        assert_eq!(app.message, M_LOST);
        Ok(())
    }
}
