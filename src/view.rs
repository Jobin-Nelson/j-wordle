use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    App,
    app::GuessType,
    constants::{
        BLOCK_SPACE_H, BLOCK_SPACE_V, C_HORIZONTAL, C_VERTICAL, LOGO, LOGO_SPACE, N_LETTERS,
        TEXT_SPACE, TRIES,
    },
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(
            Constraint::Length(C_HORIZONTAL),
            Constraint::Length(C_VERTICAL),
        );
        let main_layout = Layout::vertical([
            Constraint::Length(LOGO_SPACE as u16),
            Constraint::Fill(1),
            Constraint::Length(TEXT_SPACE as u16),
        ]);
        let [logo_area, main_area, text_area] = centered_area.layout(&main_layout);
        let try_layout = Layout::vertical([Constraint::Length(BLOCK_SPACE_V as u16); TRIES]);
        let letter_layout =
            Layout::horizontal([Constraint::Length(BLOCK_SPACE_H as u16); N_LETTERS]);
        let try_areas: [Rect; TRIES] = main_area.layout(&try_layout);
        let try_letter_areas: [[Rect; N_LETTERS]; TRIES] =
            try_areas.map(|a| a.layout(&letter_layout));

        Text::raw(LOGO).centered().render(logo_area, buf);

        let letter_block = Block::bordered().border_type(BorderType::Rounded);
        Paragraph::new(self.message)
            .alignment(Alignment::Center)
            .block(letter_block.clone())
            .render(text_area, buf);

        for (t, letter_areas) in (0..TRIES).zip(try_letter_areas) {
            for (c, letter_area) in (0..N_LETTERS).zip(letter_areas) {
                let adjusted_id = (t * N_LETTERS) + c;
                let current_char = self.attempts[adjusted_id];
                let upper_current_char = if current_char.is_ascii_alphanumeric() {
                    current_char.to_uppercase().to_string()
                } else {
                    String::from(" ")
                };

                let block_color = match self.guess_types[adjusted_id] {
                    GuessType::Green => Color::Green,
                    GuessType::Yellow => Color::Yellow,
                    GuessType::Gray => Color::Gray,
                    GuessType::NotAttempted => Color::White,
                };
                let block_style = Style::new().fg(block_color);

                Paragraph::new(upper_current_char)
                    .alignment(Alignment::Center)
                    .block(letter_block.clone().border_style(block_style))
                    .render(letter_area, buf);
            }
        }
    }
}
