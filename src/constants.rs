use indoc::indoc;

pub const N_LETTERS: usize = 5;
pub const TRIES: usize = 5;
pub const ATTEMPTS: usize = N_LETTERS * TRIES;

// View
pub const BLOCK_SPACE_H: usize = 5;
pub const BLOCK_SPACE_V: usize = 3;
pub const TEXT_SPACE: usize = 3;
pub const LOGO_SPACE: usize = 4;
pub const C_HORIZONTAL: u16 = (BLOCK_SPACE_H * N_LETTERS) as u16;
pub const C_VERTICAL: u16 = (BLOCK_SPACE_V * TRIES + TEXT_SPACE + LOGO_SPACE) as u16;

pub const M_UNFILLED: &str = "WARN: UNFILLED BLOCKS";
pub const M_WON: &str = "WON!!!";

pub const LOGO: &str = indoc! {"
█ █ █ █▀█ █▀█ █▀▄ █   █▀▀
▀▄▀▄▀ █▄█ █▀▄ █▄▀ █▄▄ ██▄
"};
