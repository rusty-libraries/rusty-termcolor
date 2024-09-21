use crate::colors::{Color, RESET};
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Struct to hold settings for various text effects.
pub struct EffectSettings {
    pub delay: u64,        // Delay between iterations in milliseconds
    pub iterations: usize, // Number of iterations for effects
    pub width: usize,      // Width for effects like loading bar
}

impl Default for EffectSettings {
    /// Provides default settings for effects.
    #[inline(always)]
    fn default() -> Self {
        Self {
            delay: 50,
            iterations: 3,
            width: 20,
        }
    }
}

/// Displays text with a typewriter effect.
///
/// # Arguments
///
/// * `text` - The text to display
/// * `settings` - EffectSettings for customization
/// * `color` - Optional color for the text
pub fn typewriter(text: &str, settings: &EffectSettings, color: Option<&Color>) {
    for c in text.chars() {
        if let Some(col) = color {
            print!("{col}{c}");
        } else {
            print!("{c}");
        }
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(settings.delay));
    }
    if color.is_some() {
        print!("{RESET}");
    }
    io::stdout().flush().unwrap();
}

/// Displays a loading bar effect.
///
/// # Arguments
///
/// * `total` - Total number of steps in the loading process
/// * `settings` - EffectSettings for customization
/// * `color` - Color for the loading bar
pub fn loading_bar(total: usize, settings: &EffectSettings, color: &Color) {
    for i in 0..=total {
        let progress = (i as f32 / total as f32 * settings.width as f32) as usize;
        print!(
            "\r{color}[{:▓>progress$}{:░>remaining$}] {i}/{total}{RESET}",
            "",
            "",
            progress = progress,
            remaining = settings.width - progress
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(settings.delay));
    }
    println!();
}

/// Displays text with a wiggle effect.
///
/// # Arguments
///
/// * `text` - The text to display
/// * `settings` - EffectSettings for customization
/// * `color` - Optional color for the text
pub fn wiggle(text: &str, settings: &EffectSettings, color: Option<&Color>) {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    for _ in 0..settings.iterations {
        for i in 0..len {
            let mut line = String::new();
            for (j, &c) in chars.iter().enumerate() {
                if j == i {
                    line.push(c.to_uppercase().next().unwrap_or(c));
                } else {
                    line.push(c.to_lowercase().next().unwrap_or(c));
                }
            }
            if let Some(col) = color {
                print!("\r{col}{line}");
            } else {
                print!("\r{line}");
            }
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(settings.delay));
        }
    }

    if color.is_some() {
        print!("{RESET}");
    }
    println!();
    io::stdout().flush().unwrap();
}

/// Displays text with a matrix-like decoding effect.
///
/// # Arguments
///
/// * `text` - The text to display
/// * `settings` - EffectSettings for customization
/// * `color` - Optional color for the text
pub fn matrix_effect(text: &str, settings: &EffectSettings, color: Option<&Color>) {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = text.chars().collect();
    let symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    for _ in 0..settings.iterations {
        for i in 0..chars.len() {
            let mut line = String::new();
            for (j, &c) in chars.iter().enumerate() {
                if j <= i {
                    line.push(c);
                } else {
                    line.push(
                        symbols
                            .chars()
                            .nth(rng.gen_range(0..symbols.len()))
                            .unwrap(),
                    );
                }
            }
            if let Some(col) = color {
                print!("\r{col}{line}");
            } else {
                print!("\r{line}");
            }
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(settings.delay));
        }
    }

    if color.is_some() {
        print!("{RESET}");
    }
    println!();
    io::stdout().flush().unwrap();
}

/// Displays text with a rainbow effect.
///
/// # Arguments
///
/// * `text` - The text to display
/// * `settings` - EffectSettings for customization
pub fn rainbow_text(text: &str, settings: &EffectSettings) {
    let colors = [
        Color::new(255, 0, 0),   // Red
        Color::new(255, 127, 0), // Orange
        Color::new(255, 255, 0), // Yellow
        Color::new(0, 255, 0),   // Green
        Color::new(0, 0, 255),   // Blue
        Color::new(75, 0, 130),  // Indigo
        Color::new(143, 0, 255), // Violet
    ];

    for _ in 0..settings.iterations {
        for i in 0..colors.len() {
            let mut colored_text = String::new();
            for (j, c) in text.chars().enumerate() {
                let color_index = (i + j) % colors.len();
                colored_text.push_str(&format!("{}{c}", colors[color_index]));
            }
            print!("\r{colored_text}{RESET}");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(settings.delay));
        }
    }
    println!();
}

/// Displays a progress spinner with customizable styles.
///
/// # Arguments
///
/// * `total` - Total number of steps in the process
/// * `settings` - EffectSettings for customization
/// * `color` - Color for the spinner
/// * `style` - Style of the spinner (0: default, 1: dots, 2: arrows)
pub fn progress_spinner(total: usize, settings: &EffectSettings, color: &Color, style: usize) {
    let spinner_chars = match style {
        1 => vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
        2 => vec!['←', '↖', '↑', '↗', '→', '↘', '↓', '↙'],
        _ => vec!['|', '/', '-', '\\'],
    };

    for i in 0..=total {
        let spinner_char = spinner_chars[i % spinner_chars.len()];
        print!("\r{color}{spinner_char} {i}/{total}");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(settings.delay));
    }
    println!("{RESET}");
}
