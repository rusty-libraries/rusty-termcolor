use std::io::{self, Write};

/// Clears the terminal screen and moves the cursor to the top-left corner.
pub fn clear_screen() {
    // ANSI escape code to clear screen and move cursor to (1,1)
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Sets the terminal window title.
///
/// # Arguments
///
/// * `title` - The string to set as the terminal window title
pub fn set_title(title: &str) {
    // ANSI escape code to set terminal title
    print!("\x1B]0;{}\x07", title);
    io::stdout().flush().unwrap();
}

/// Hides the cursor in the terminal.
pub fn hide_cursor() {
    // ANSI escape code to hide cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}

/// Shows the cursor in the terminal.
pub fn show_cursor() {
    // ANSI escape code to show cursor
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}