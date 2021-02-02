/// Create new Color instance.
#[allow(non_camel_case_types)]
pub struct new(pub RGB);

impl new {
    /// Get Color value as RGB.
    pub fn value(&self) -> RGB {
        self.0
    }
    /// Inverted color value.
    pub fn inverted(&mut self) -> &mut Self {
        self.0 = inverted(self.0);
        self
    }
    /// Get Color hex value.
    pub fn to_hex(&self) -> String {
        rgb_to_hex(self.0)
    }
    /// Get contrast ratio between two colors.
    pub fn contrast_ratio(&self, rgb: RGB) -> f64 {
        contrast_ratio(rgb, self.0)
    }
    pub fn luminance(&self) -> f64 {
        luminance(self.0)
    }
}
/// Create RGB instance from hex value.
/// ### Examples
/// ```
/// Color::from_hex("#00ff00").unwrap();
/// ```
// ********************
pub fn from_hex(hex: &str) -> Result<new, &str> {
    match hex_to_rgb(hex) {
        Ok(vec) => {
            if let Ok(rgb) = vec_to_RGB(vec) {
                let color_instense = new(rgb);
                Ok(color_instense)
            } else {
                Err("Invalid")
            }
        }
        Err(msg) => Err(msg),
    }
}