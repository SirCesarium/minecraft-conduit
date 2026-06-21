#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        println!(
            "{reset}{bg}{fg}{bold} X {reset} {msg}{clear}",
            reset = $crate::display::palette::TAG_RESET,
            bg = $crate::display::palette::ERROR_TAG_BG,
            fg = $crate::display::palette::TAG_FG,
            bold = $crate::display::palette::TAG_BOLD,
            msg = format!($($arg)*),
            clear = $crate::display::palette::CLEAR,
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        println!(
            "{reset}{bg}{fg}{bold} ⚠ {reset} {msg}{clear}",
            reset = $crate::display::palette::TAG_RESET,
            bg = $crate::display::palette::WARN_TAG_BG,
            fg = $crate::display::palette::TAG_FG,
            bold = $crate::display::palette::TAG_BOLD,
            msg = format!($($arg)*),
            clear = $crate::display::palette::CLEAR,
        );
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        println!(
            "{reset}{bg}{fg}{bold} ℹ {reset} {msg}{clear}",
            reset = $crate::display::palette::TAG_RESET,
            bg = $crate::display::palette::INFO_TAG_BG,
            fg = $crate::display::palette::TAG_FG,
            bold = $crate::display::palette::TAG_BOLD,
            msg = format!($($arg)*),
            clear = $crate::display::palette::CLEAR,
        );
    }};
}

pub mod palette;
pub mod progress;
pub mod spinner;
