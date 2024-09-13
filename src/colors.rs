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
    ///
    /// # Returns
    ///
    /// A new Color instance.
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

    /// Converts the Color to a 256-color code.
    ///
    /// # Returns
    ///
    /// A u8 representing the closest 256-color code.
    pub fn to_256_color(&self) -> u8 {
        let (r, g, b) = self.rgb();
        16 + 36 * (r as u16 * 5 / 255) as u8 + 6 * (g as u16 * 5 / 255) as u8 + (b as u16 * 5 / 255) as u8
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

/// Returns a random aesthetically pleasing color.
///
/// # Returns
///
/// A random Color instance.
pub fn random_pleasing_color() -> Color {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let hue = rng.gen_range(0..360) as f32;
    let saturation = rng.gen_range(70..100) as f32 / 100.0;
    let value = rng.gen_range(70..100) as f32 / 100.0;
    
    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match (hue as u16) / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}