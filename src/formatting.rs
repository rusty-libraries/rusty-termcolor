use crate::colors::{Color, RESET};
use std::io::{self, Write};
use terminal_size::{terminal_size, Width};

/// Prints colored text without a newline.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `color` - The color to use for the text
pub fn print_colored(text: &str, color: &Color) {
    print!("{color}{text}{RESET}");
    io::stdout().flush().unwrap();
}

/// Prints colored text with a newline.
///
/// # Arguments
///
/// * `text` - The text to print
/// * `color` - The color to use for the text
pub fn println_colored(text: &str, color: &Color) {
    println!("{color}{text}{RESET}");
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
        print!("{}{c}", colors[color_index]);
    }

    print!("{RESET}");
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
    let width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
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

/// Creates a styled table with borders and cell formatting.
///
/// # Arguments
///
/// * `headers` - A slice of strings representing the table headers
/// * `rows` - A [`Vec`] of [`Vec`]s of strings representing the table data
/// * `color` - Optional color for the table borders and headers
///
/// # Returns
///
/// A String containing the formatted table
pub fn create_table(headers: &[&str], rows: &Vec<Vec<String>>, color: Option<&Color>) -> String {
    let column_widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, &header)| {
            rows.iter()
                .map(|row| row.get(i).map_or(0, |cell| cell.len()))
                .max()
                .unwrap_or(0)
                .max(header.len())
        })
        .collect();

    let mut table = String::new();
    let color_str = color.map_or_else(String::new, |c| c.to_string());
    let reset_str = color.map_or_else(String::new, |_| crate::colors::RESET.to_string());

    // Top border
    table.push_str(&format!(
        "{color_str}╔{}╗{reset_str}\n",
        column_widths
            .iter()
            .map(|&w| "═".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("╦")
    ));

    // Headers
    table.push_str(&format!("{color_str}║ "));
    for (i, header) in headers.iter().enumerate() {
        table.push_str(&format!("{:<width$} ", header, width = column_widths[i]));
        if i < headers.len() - 1 {
            table.push_str("│ ");
        }
    }
    table.push_str(&format!("║{reset_str}\n"));

    // Separator
    table.push_str(&format!(
        "{color_str}╠{}╣{reset_str}\n",
        column_widths
            .iter()
            .map(|&w| "═".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("╬")
    ));

    // Rows
    for row in rows {
        table.push_str(&format!("{color_str}║ "));
        for (i, cell) in row.iter().enumerate() {
            table.push_str(&format!("{:<width$} ", cell, width = column_widths[i]));
            if i < row.len() - 1 {
                table.push_str("│ ");
            }
        }
        table.push_str(&format!("║{reset_str}\n"));
    }

    // Bottom border
    table.push_str(&format!(
        "{color_str}╚{}╝{reset_str}\n",
        column_widths
            .iter()
            .map(|&w| "═".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("╩")
    ));

    table
}
