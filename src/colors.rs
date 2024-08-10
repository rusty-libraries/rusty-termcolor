use std::fmt;

/// Represents an RGB color.
pub struct Color {
    r: u8,  // Red component (0-255)
    g: u8,  // Green component (0-255)
    b: u8,  // Blue component (0-255)
}

impl Color {
    /// Creates a new Color instance.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Returns the RGB components as a tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing the (red, green, blue) components.
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl fmt::Display for Color {
    /// Formats the Color as an ANSI escape sequence for terminal output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

// Predefined color constants
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

/// ANSI escape sequence to reset text formatting.
pub const RESET: &str = "\x1B[0m";

/// Generates a color gradient between two colors.
///
/// # Arguments
///
/// * `start` - The starting color of the gradient
/// * `end` - The ending color of the gradient
/// * `steps` - The number of color steps in the gradient
///
/// # Returns
///
/// A vector of Colors representing the gradient from start to end.
pub fn fade_color(start: &Color, end: &Color, steps: usize) -> Vec<Color> {
    let (r1, g1, b1) = start.rgb();
    let (r2, g2, b2) = end.rgb();
    
    (0..steps).map(|i| {
        let t = i as f32 / (steps - 1) as f32;
        let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
        let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
        let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
        Color::new(r, g, b)
    }).collect()
}