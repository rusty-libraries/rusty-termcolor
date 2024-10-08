use crate::formatting::center_text;

/// Represents a banner with ASCII art and text.
pub struct Banner<'a, 'b> {
    ascii_art: &'a str, // The ASCII art to be displayed
    text: &'b str,      // The text to be displayed alongside the ASCII art
    padding: usize,     // The amount of padding between ASCII art and text
    position: Position, // The position of the text relative to the ASCII art
}

/// Enum representing the possible positions of the text relative to the ASCII art.
pub enum Position {
    Top,
    Middle,
    Bottom,
}

impl<'a, 'b> Banner<'a, 'b> {
    /// Creates a new [`Banner`] instance.
    ///
    /// # Arguments
    ///
    /// * `ascii_art` - The ASCII art to be displayed.
    /// * `text` - The text to be displayed alongside the ASCII art.
    /// * `padding` - The amount of padding between ASCII art and text.
    /// * `position` - The position of the text relative to the ASCII art.
    pub const fn new(
        ascii_art: &'a str,
        text: &'b str,
        padding: usize,
        position: Position,
    ) -> Self {
        Self {
            ascii_art,
            text,
            padding,
            position,
        }
    }

    /// Renders the banner, combining ASCII art and text.
    ///
    /// # Returns
    ///
    /// A String containing the rendered banner.
    pub fn render(&self) -> String {
        let ascii_lines = self.ascii_art.lines().collect::<Vec<_>>();
        let text_lines = self.text.lines().collect::<Vec<_>>();

        let ascii_width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0);

        let mut result = String::new();

        let total_lines = ascii_lines.len();
        let text_start = match self.position {
            Position::Top => 0,
            Position::Middle => (total_lines - text_lines.len()) / 2,
            Position::Bottom => total_lines - text_lines.len(),
        };

        for i in 0..total_lines {
            let ascii_line = ascii_lines[i];
            let text_line = if i >= text_start && i < text_start + text_lines.len() {
                text_lines[i - text_start]
            } else {
                ""
            };

            let centered_text = center_text(text_line);
            let padding = " ".repeat(self.padding);

            let line = format!(
                "{:<ascii_width$}{padding}{centered_text}",
                ascii_line,
                ascii_width = ascii_width
            );

            result.push_str(&line);
            result.push('\n');
        }

        result.trim_end().to_string()
    }
}

/// Creates and renders a banner with the given parameters.
///
/// # Arguments
///
/// * `ascii_art` - The ASCII art to be displayed.
/// * `text` - The text to be displayed alongside the ASCII art.
/// * `padding` - The amount of padding between ASCII art and text.
/// * `position` - The position of the text relative to the ASCII art.
///
/// # Returns
///
/// A String containing the rendered banner.
pub fn create_banner(ascii_art: &str, text: &str, padding: usize, position: Position) -> String {
    let banner = Banner::new(ascii_art, text, padding, position);
    banner.render()
}
