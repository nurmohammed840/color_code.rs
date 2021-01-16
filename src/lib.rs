#![allow(non_snake_case)]

/// Get luminance from rgb.
/// ### Examples
/// ```
/// let lum = Color::luminance([255, 255, 255]);
/// assert_eq!(1.0, lum);
/// ```
pub fn luminance(rgb: [u8; 3]) -> f64 {
    let mut arr = [0.; 3];
    for i in 0..=2 {
        let color = (rgb[i] as f64) / 255.;
        arr[i] = if color <= 0.03928 {
            color / 12.92
        } else {
            ((color + 0.055) / 1.055).powf(2.4)
        };
    }
    arr[0] * 0.2126 + arr[1] * 0.7152 + arr[2] * 0.0722
}

/// Create RGB instance.
pub struct RGB(pub [u8; 3]);

impl RGB {
    /// Get RGB value.
    pub fn value(&self) -> [u8; 3] {
        self.0
    }
    /// Inverted rgb value.
    pub fn inverted(&mut self) -> &mut Self {
        let rgb = &mut self.0;
        for color in rgb.iter_mut() {
            *color = 255 - *color;
        }
        self
    }
    /// convart rgb value to hex value.
    pub fn to_hex(&self) -> String {
        let rgb = self.0;
        format!("{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
    }
    /// Get contrast ratio between two colors.
    pub fn contrast_ratio(&self, rgb: [u8; 3]) -> f64 {
        let lum1 = luminance(rgb);
        let lum2 = luminance(self.0);
        (lum1.max(lum2) + 0.05) / (lum1.min(lum2) + 0.05)
    }
    /// Create RGB instance from hex value.
    /// ### Examples
    /// ```
    /// RGB::from_hex("#00ff00").unwrap();
    /// ```
    pub fn from_hex(hex_value: &str) -> Result<Self, &str> {
        let hex = hex_value.trim_start_matches("#");
        let mut rgb = [0u8; 3];
        if hex.len() != 6 {
            return Err("Invalid Color");
        }
        for i in 0..=2 {
            let x = i * 2;
            match u8::from_str_radix(&hex[x..(x + 2)], 16) {
                Ok(color) => rgb[i] = color,
                Err(_) => return Err("Invalid Digit"),
            }
        }
        Ok(Self(rgb))
    }
}