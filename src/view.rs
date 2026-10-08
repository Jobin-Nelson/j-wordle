use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    App,
    constants::{BLOCK_SPACE_H, BLOCK_SPACE_V, N_LETTERS, TRIES},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(
            Constraint::Length((BLOCK_SPACE_H * N_LETTERS) as u16),
            Constraint::Length((BLOCK_SPACE_V * TRIES) as u16),
        );
        let try_layout = Layout::vertical([Constraint::Length(BLOCK_SPACE_V as u16); TRIES]);
        let letter_layout =
            Layout::horizontal([Constraint::Length(BLOCK_SPACE_H as u16); N_LETTERS]);
        let try_areas: [Rect; TRIES] = centered_area.layout(&try_layout);
        let try_letter_areas: [[Rect; N_LETTERS]; TRIES] =
            try_areas.map(|a| a.layout(&letter_layout));

        let letter_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().white());

        for (word, letter_areas) in self.attempts.chunks_exact(N_LETTERS).zip(try_letter_areas) {
            for (c, letter_area) in word.iter().zip(letter_areas) {
                let rc = if c.is_ascii_alphanumeric() {
                    c.to_uppercase().to_string()
                } else {
                    String::from(" ")
                };

                Paragraph::new(rc)
                    .alignment(Alignment::Center)
                    .block(letter_block.clone())
                    .render(letter_area, buf);
            }
        }
    }
}
