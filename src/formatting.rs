use crate::colors::Color;
use std::io::{self, Write};
use terminal_size::{Width, terminal_size};

/// Prints colored text without a newline.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `color` - The color to use for the text
pub fn print_colored(text: &str, color: &Color) {
    print!("{}{}{}", color, text, crate::colors::RESET);
    io::stdout().flush().unwrap();
}

/// Prints colored text with a newline.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `color` - The color to use for the text
pub fn println_colored(text: &str, color: &Color) {
    println!("{}{}{}", color, text, crate::colors::RESET);
}

/// Prints text with a color gradient effect.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `colors` - An array of colors to use for the gradient
pub fn print_fade(text: &str, colors: &[Color]) {
    let chars: Vec<char> = text.chars().collect();
    let color_count = colors.len();
    
    for (i, c) in chars.iter().enumerate() {
        let color_index = (i * color_count) / chars.len();
        print!("{}{}", colors[color_index], c);
    }
    print!("{}", crate::colors::RESET);
    io::stdout().flush().unwrap();
}

/// Centers text based on the terminal width.
///
/// # Arguments
///
/// * `text` - The text to center
///
/// # Returns
///
/// A String containing the centered text
pub fn center_text(text: &str) -> String {
    let width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let padding = (width - text.len()) / 2;
    format!("{:>width$}", text, width = padding + text.len())
}

/// Surrounds text with a box made of Unicode box-drawing characters.
///
/// # Arguments
///
/// * `text` - The text to put in a box
///
/// # Returns
///
/// A String containing the text surrounded by a box
pub fn box_text(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let max_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let top_bottom = format!("╔{}╗", "═".repeat(max_length + 2));
    let mut result = String::new();

    result.push_str(&top_bottom);
    result.push('\n');

    for line in lines {
        result.push_str(&format!("║ {:<width$} ║\n", line, width = max_length));
    }

    result.push_str(&top_bottom.replace("╔", "╚").replace("╗", "╝"));
    result
}