# rusty-termcolor

## Overview

This crate provides a set of utility functions for terminal manipulation and text formatting. It includes functionality for color manipulation, text effects, terminal control, and text formatting.

## Modules

### colors

This module defines color-related structures and functions.

#### Struct: `Color`

Represents an RGB color.

##### Fields:
- `r: u8` - Red component (0-255)
- `g: u8` - Green component (0-255)
- `b: u8` - Blue component (0-255)

##### Methods:
- `new(r: u8, g: u8, b: u8) -> Self` - Creates a new Color instance.
- `rgb(&self) -> (u8, u8, u8)` - Returns the RGB components as a tuple.

##### Implementation:
- `fmt::Display` - Formats the Color as an ANSI escape sequence for terminal output.

#### Constants:
- `RED`, `GREEN`, `BLUE`, `YELLOW`, `MAGENTA`, `CYAN`, `WHITE`, `BLACK` - Predefined color constants.
- `RESET: &str` - ANSI escape sequence to reset text formatting.

#### Functions:
- `fade_color(start: &Color, end: &Color, steps: usize) -> Vec<Color>` - Generates a color gradient between two colors.

### effects

This module provides various text effects and animations.

#### Struct: `EffectSettings`

Holds settings for various text effects.

##### Fields:
- `delay: u64` - Delay between iterations in milliseconds.
- `iterations: usize` - Number of iterations for effects.
- `width: usize` - Width for effects like loading bar.

##### Implementation:
- `Default` - Provides default settings for effects.

#### Functions:
- `typewriter(text: &str, settings: &EffectSettings, color: Option<&Color>)` - Displays text with a typewriter effect.
- `loading_bar(total: usize, settings: &EffectSettings, color: &Color)` - Displays a loading bar effect.
- `wiggle(text: &str, settings: &EffectSettings, color: Option<&Color>)` - Displays text with a wiggle effect.
- `matrix_effect(text: &str, settings: &EffectSettings, color: Option<&Color>)` - Displays text with a matrix-like decoding effect.

### formatting

This module provides text formatting utilities.

#### Functions:
- `print_colored(text: &str, color: &Color)` - Prints colored text without a newline.
- `println_colored(text: &str, color: &Color)` - Prints colored text with a newline.
- `print_fade(text: &str, colors: &[Color])` - Prints text with a color gradient effect.
- `center_text(text: &str) -> String` - Centers text based on the terminal width.
- `box_text(text: &str) -> String` - Surrounds text with a box made of Unicode box-drawing characters.

### terminal

This module provides terminal control functions.

#### Functions:
- `clear_screen()` - Clears the terminal screen and moves the cursor to the top-left corner.
- `set_title(title: &str)` - Sets the terminal window title.
- `hide_cursor()` - Hides the cursor in the terminal.
- `show_cursor()` - Shows the cursor in the terminal.

## Usage Examples

```rust
use rusty_termcolor::{
    colors::*, formatting::*, effects::*, system::*,
    Color, fade_color,
};
use std::{thread, time::Duration};
use rusty_termcolor::styles::banners::{create_banner, Position};

fn main() {
    // System functions
    set_title("Rusty TermColor Demo");
    clear_screen();

    let default_settings = EffectSettings::default();
    let fast_settings = EffectSettings { delay: 20, ..default_settings };
    let slow_settings = EffectSettings { delay: 100, ..default_settings };

    // Basic colored text
    println_colored("Welcome to Rusty TermColor!", &RED);
    thread::sleep(Duration::from_secs(1));

    // Fade effect
    let fade_colors = fade_color(&BLUE, &GREEN, 10);
    print_fade("This text fades from blue to green\n", &fade_colors);
    thread::sleep(Duration::from_secs(1));

    // Typewriter effect
    typewriter("This text appears like it's being typed...\n", &default_settings, Some(&YELLOW));
    thread::sleep(Duration::from_secs(1));

    // Wiggle effect
    wiggle("This text wiggles!", &default_settings, Some(&CYAN));
    thread::sleep(Duration::from_secs(1));

    // Matrix effect
    matrix_effect("Matrix effect demo", &default_settings, Some(&GREEN));
    thread::sleep(Duration::from_secs(1));

    // Centered text
    println!("{}", center_text("This text is centered"));
    thread::sleep(Duration::from_secs(1));

    // Box text
    let boxed_text = box_text("This text\nis inside\na box!");
    println_colored(&boxed_text, &CYAN);
    thread::sleep(Duration::from_secs(1));


    // Banner
    let ascii_art = r#"
  _____  _    _  _____ _________     __
 |  __ \| |  | |/ ____|__   __\ \   / /
 | |__) | |  | | (___    | |   \ \_/ / 
 |  _  /| |  | |\___ \   | |    \   /  
 | | \ \| |__| |____) |  | |     | |   
 |_|  \_\\____/|_____/   |_|     |_|   
"#;
    let banner_text = "Welcome to Rusty TermColor!";
    let banner = create_banner(ascii_art, banner_text, 2, Position::Bottom);
    println_colored(&banner, &MAGENTA);
    thread::sleep(Duration::from_secs(1));

    // Loading bar
    println!("Loading (default speed)...");
    loading_bar(20, &default_settings, &GREEN);
    thread::sleep(Duration::from_secs(1));

    println!("Loading (fast)...");
    loading_bar(20, &fast_settings, &YELLOW);
    thread::sleep(Duration::from_secs(1));

    println!("Loading (slow)...");
    loading_bar(20, &slow_settings, &RED);
    thread::sleep(Duration::from_secs(1));

    // Hide and show cursor
    hide_cursor();
    println!("The cursor is now hidden.");
    thread::sleep(Duration::from_secs(2));
    show_cursor();
    println!("The cursor is now visible again.");

    // Custom color
    let custom_color = Color::new(100, 150, 200);
    println_colored("This text uses a custom color!", &custom_color);

    // Showcase different speeds for effects
    println!("\nShowcasing different speeds for effects:");

    println!("\nTypewriter (fast):");
    typewriter("Fast typewriter effect", &fast_settings, Some(&BLUE));

    println!("\nTypewriter (slow):");
    typewriter("Slow typewriter effect", &slow_settings, Some(&RED));

    println!("\nWiggle (fast):");
    wiggle("Fast wiggle effect", &fast_settings, Some(&GREEN));

    println!("\nWiggle (slow):");
    wiggle("Slow wiggle effect", &slow_settings, Some(&YELLOW));

    println!("\nMatrix (fast):");
    matrix_effect("Fast matrix effect", &fast_settings, Some(&CYAN));

    println!("\nMatrix (slow):");
    matrix_effect("Slow matrix effect", &slow_settings, Some(&MAGENTA));

    println!("\nDemo completed. Press Enter to exit.");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).unwrap();
}
```

## Notes

- Some functions use ANSI escape codes, which may not be supported on all terminals or operating systems.
- The effectiveness of visual effects may vary depending on the terminal emulator and system configuration.
- It's recommended to use `show_cursor()` before exiting the program if `hide_cursor()` was used.

## Dependencies

- `std::io` for terminal I/O operations.
- `std::thread` and `std::time::Duration` for timing in effects.
- `rand` crate for random number generation in some effects.
- `terminal_size` crate for getting terminal dimensions.