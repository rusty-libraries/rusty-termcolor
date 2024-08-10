# Rusty TermColor

Rusty TermColor is a Rust crate that provides utility functions for terminal manipulation and text formatting. It includes functionality for color manipulation, text effects, terminal control, and text formatting.

## Table of Contents

1. [Installation](#installation)
2. [Features](#features)
3. [Usage](#usage)
4. [Modules](#modules)
   - [Colors](#colors)
   - [Effects](#effects)
   - [Formatting](#formatting)
   - [Terminal](#terminal)
5. [Examples](#examples)
6. [Notes](#notes)
7. [Dependencies](#dependencies)
8. [License](#license)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusty-termcolor = "0.1.1"
```

## Features

- Color manipulation and predefined color constants
- Text effects (typewriter, loading bar, wiggle, matrix)
- Text formatting (colored output, gradients, centering, boxing)
- Terminal control (clear screen, set title, hide/show cursor)

## Usage

```rust
use rusty_termcolor::{colors::*, formatting::*, effects::*, terminal::*};

fn main() {
    println_colored("Hello, Rusty TermColor!", &RED);
    typewriter("This appears like it's being typed...", &EffectSettings::default(), Some(&GREEN));
    clear_screen();
    // ... see docs for more examples
}
```

## Modules

### Colors

- `Color` struct for RGB color representation
- Predefined color constants
- Color gradient generation

### Effects

- `EffectSettings` struct for customizing effect parameters
- Typewriter effect
- Loading bar
- Wiggle effect
- Matrix effect

### Formatting

- Colored text output
- Gradient text
- Text centering
- Text boxing

### Terminal

- Clear screen
- Set terminal title
- Hide/show cursor

## Examples

See the [Usage Examples](#usage-examples) section in the detailed documentation for comprehensive examples of using Rusty TermColor's features.

## Notes

- Some functions use ANSI escape codes, which may not be supported on all terminals or operating systems.
- The effectiveness of visual effects may vary depending on the terminal emulator and system configuration.
- Remember to use `show_cursor()` before exiting if `hide_cursor()` was used.

## Dependencies

- `std::io` for terminal I/O operations
- `std::thread` and `std::time::Duration` for timing in effects
- `rand` crate for random number generation in some effects
- `terminal_size` crate for getting terminal dimensions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.