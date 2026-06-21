use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::display::palette;

fn template(s: &str) -> ProgressStyle {
    ProgressStyle::with_template(s).unwrap_or_else(|_| panic!("invalid template: {s:?}"))
}

pub fn new_spinner(msg: &str) -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.set_style(template(palette::SPINNER_TEMPLATE).tick_chars("◐◓◑◒◐"));
    bar.set_message(msg.to_string());
    bar.enable_steady_tick(Duration::from_millis(120));
    bar
}

pub fn finish_spinner(bar: &ProgressBar, msg: &str) {
    bar.set_style(template(palette::DONE_TEMPLATE));
    bar.finish_with_message(format!(
        "{fg}✓{reset} {msg}",
        fg = palette::DONE_FG,
        reset = palette::RESET,
    ));
}
