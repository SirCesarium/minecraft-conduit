use std::fmt::Display;

use inquire::error::InquireResult;
use inquire::{Confirm, Select, Text};

pub fn text(prompt: &str) -> InquireResult<String> {
    Text::new(prompt).prompt()
}

pub fn text_with_default(prompt: &str, default: &str) -> InquireResult<String> {
    Text::new(prompt).with_default(default).prompt()
}

pub fn select<T>(prompt: &str, options: Vec<T>) -> InquireResult<T>
where
    T: Display + Clone + PartialEq,
{
    Select::new(prompt, options).prompt()
}

pub fn confirm(prompt: &str, default: bool) -> InquireResult<bool> {
    Confirm::new(prompt).with_default(default).prompt()
}
