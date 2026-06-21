// Gruvbox dark medium true-color palette.
//
// Macros and indicatif templates use 24-bit colour escapes so pywal/theme
// overrides don't affect the colours.
//
// https://github.com/morhetz/gruvbox

// ── Control sequences ──

pub const RESET: &str = "\x1b[0m";
pub const CLEAR: &str = "\x1b[K";
pub const BOLD: &str = "\x1b[1m";

// ── Foreground colours (24-bit true color) ──

macro_rules! fg {
    ($r:expr, $g:expr, $b:expr) => {
        concat!("\x1b[38;2;", $r, ";", $g, ";", $b, "m")
    };
}

pub const FG_BG0: &str = fg!(40, 40, 40); // #282828
pub const FG_RED: &str = fg!(204, 36, 29); // #cc241d
pub const FG_GREEN: &str = fg!(152, 151, 26); // #98971a
pub const FG_YELLOW: &str = fg!(215, 153, 33); // #d79921
pub const FG_BLUE: &str = fg!(69, 133, 136); // #458588
pub const FG_PURPLE: &str = fg!(177, 98, 134); // #b16286
pub const FG_AQUA: &str = fg!(104, 157, 106); // #689d6a
pub const FG_GRAY: &str = fg!(146, 131, 116); // #928374
pub const FG_ORANGE: &str = fg!(214, 93, 14); // #d65d0e
pub const FG_FG: &str = fg!(235, 219, 178); // #ebdbb2

pub const FG_BRIGHT_RED: &str = fg!(251, 73, 52); // #fb4934
pub const FG_BRIGHT_GREEN: &str = fg!(184, 187, 38); // #b8bb26
pub const FG_BRIGHT_YELLOW: &str = fg!(250, 189, 47); // #fabd2f
pub const FG_BRIGHT_BLUE: &str = fg!(131, 165, 152); // #83a598
pub const FG_BRIGHT_PURPLE: &str = fg!(211, 134, 155); // #d3869b
pub const FG_BRIGHT_AQUA: &str = fg!(142, 192, 124); // #8ec07c
pub const FG_BRIGHT_ORANGE: &str = fg!(254, 128, 25); // #fe8019

// ── Background colours (24-bit true color) ──

macro_rules! bg {
    ($r:expr, $g:expr, $b:expr) => {
        concat!("\x1b[48;2;", $r, ";", $g, ";", $b, "m")
    };
}

pub const BG_BG0: &str = bg!(40, 40, 40); // #282828
pub const BG_RED: &str = bg!(204, 36, 29); // #cc241d
pub const BG_GREEN: &str = bg!(152, 151, 26); // #98971a
pub const BG_YELLOW: &str = bg!(215, 153, 33); // #d79921
pub const BG_BLUE: &str = bg!(69, 133, 136); // #458588
pub const BG_PURPLE: &str = bg!(177, 98, 134); // #b16286
pub const BG_AQUA: &str = bg!(104, 157, 106); // #689d6a
pub const BG_GRAY: &str = bg!(146, 131, 116); // #928374
pub const BG_ORANGE: &str = bg!(214, 93, 14); // #d65d0e
pub const BG_FG: &str = bg!(235, 219, 178); // #ebdbb2

pub const BG_BRIGHT_RED: &str = bg!(251, 73, 52); // #fb4934
pub const BG_BRIGHT_GREEN: &str = bg!(184, 187, 38); // #b8bb26
pub const BG_BRIGHT_YELLOW: &str = bg!(250, 189, 47); // #fabd2f
pub const BG_BRIGHT_BLUE: &str = bg!(131, 165, 152); // #83a598
pub const BG_BRIGHT_PURPLE: &str = bg!(211, 134, 155); // #d3869b
pub const BG_BRIGHT_AQUA: &str = bg!(142, 192, 124); // #8ec07c
pub const BG_BRIGHT_ORANGE: &str = bg!(254, 128, 25); // #fe8019

// ── Semantic aliases for log macros ──

pub const TAG_FG: &str = fg!(0, 0, 0);
pub const TAG_BOLD: &str = BOLD;
pub const TAG_RESET: &str = RESET;
pub const ERROR_TAG_BG: &str = BG_RED;
pub const WARN_TAG_BG: &str = BG_YELLOW;
pub const INFO_TAG_BG: &str = BG_AQUA;

// ── Semantic aliases for progress/spinner ──

pub const DONE_FG: &str = FG_GREEN;

// ── Indicatif template strings ──
// Uses hex colours (`#rrggbb`) via `console::Style::from_dotted_str`
// so colours match the Gruvbox palette regardless of terminal theme.

pub const PROGRESS_BAR_TEMPLATE: &str = "{msg} {bar:15.#458588/#689d6a} {percent:>3}%";
pub const SPINNER_TEMPLATE: &str = "{spinner:.#83a598} {msg}";
pub const DONE_TEMPLATE: &str = "{msg}";
