/// A percent-based RGB Color.
///
/// Values are from 0-100 NOT 0-255.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorPercent {
    /// Red
    ///
    /// Valued from 0-100
    pub r: u8,

    /// Green
    ///
    /// Valued from 0-100
    pub g: u8,

    /// Blue
    ///
    /// Valued from 0-100
    pub b: u8,
}

impl ColorPercent {
    /// Creates a new color from raw percentage values, NOT RGB values.
    ///
    /// # Returns
    /// This returns a new ColorPercent if the values are in range.
    /// This returns `None` if any of the values are out of range.
    pub fn new_percent(r: u8, g: u8, b: u8) -> Option<Self> {
        if r > 100 || g > 100 || b > 100 {
            return None;
        }

        Some(Self { r, g, b })
    }

    /// Creates a new color from RGB values, NOT percentage values.
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: ((u16::from(r) * 100) / 255) as u8,
            g: ((u16::from(g) * 100) / 255) as u8,
            b: ((u16::from(b) * 100) / 255) as u8,
        }
    }
}
