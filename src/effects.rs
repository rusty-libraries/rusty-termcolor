use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use rand::Rng;
use crate::colors::Color;

/// Struct to hold settings for various text effects.
pub struct EffectSettings {
    pub delay: u64,       // Delay between iterations in milliseconds
    pub iterations: usize, // Number of iterations for effects
    pub width: usize,     // Width for effects like loading bar
}

impl Default for EffectSettings {
    /// Provides default settings for effects.
    fn default() -> Self {
        EffectSettings {
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
            print!("{}{}", col, c);
        } else {
            print!("{}", c);
        }
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(settings.delay));
    }
    if color.is_some() {
        print!("{}", crate::colors::RESET);
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
        print!("\r{}[{:▓>progress$}{:░>remaining$}] {}/{}{}",
               color,
               "",
               "",
               i,
               total,
               crate::colors::RESET,
               progress = progress,
               remaining = settings.width - progress);
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
                print!("\r{}{}", col, line);
            } else {
                print!("\r{}", line);
            }
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(settings.delay));
        }
    }

    if color.is_some() {
        print!("{}", crate::colors::RESET);
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
                    line.push(symbols.chars().nth(rng.gen_range(0..symbols.len())).unwrap());
                }
            }
            if let Some(col) = color {
                print!("\r{}{}", col, line);
            } else {
                print!("\r{}", line);
            }
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(settings.delay));
        }
    }

    if color.is_some() {
        print!("{}", crate::colors::RESET);
    }
    println!();
    io::stdout().flush().unwrap();
}