use indoc::indoc;

// App
pub const N_LETTERS: usize = 5;
pub const ATTEMPTS: usize = 5;
pub const N_GUESSES: usize = N_LETTERS * ATTEMPTS;
pub const CORPUS: &str = include_str!("valid_solutions.txt");

// View
pub const LOGO: &str = indoc! {"
█ █ █ █▀█ █▀█ █▀▄ █   █▀▀
▀▄▀▄▀ █▄█ █▀▄ █▄▀ █▄▄ ██▄
"};
pub const BLOCK_SPACE_H: usize = 5;
pub const BLOCK_SPACE_V: usize = 3;
pub const TEXT_SPACE: usize = 3;
pub const LOGO_SPACE: usize = 4;
pub const C_HORIZONTAL: u16 = (BLOCK_SPACE_H * N_LETTERS) as u16;
pub const C_VERTICAL: u16 = (BLOCK_SPACE_V * ATTEMPTS + TEXT_SPACE + LOGO_SPACE) as u16;

// Messages
pub const M_UNFILLED: &str = "WARN: UNFILLED BLOCKS";
pub const M_WON: &str = "YOU WON!!!";
pub const M_LOST: &str = "YOU LOST, ANS: ";
