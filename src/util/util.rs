use clearscreen::clear;
use owo_colors::OwoColorize;

/// display info
pub fn display_info(text: &str) {
    println!("{}", text.to_owned().default_color());
}

/// display error message, does not panic or end execution
pub fn display_error(text: &str) {
    println!("{}", text.to_owned().bright_red());
}

/// display success message
pub fn display_success(text: &str) {
    println!("{}", text.to_owned().bright_green());
}

pub fn clear_console() {
    clear().expect("failed to clear screen");
}
