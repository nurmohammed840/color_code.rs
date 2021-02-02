use crate::color_types::RGB;

/// Get contrast ratio between two colors.
pub fn contrast_ratio(color1: RGB, color2: RGB) -> f64 {
    let lum1 = luminance(color1);
    let lum2 = luminance(color2);
    (lum1.max(lum2) + 0.05) / (lum1.min(lum2) + 0.05)
}
/// Inverted color value.
pub fn inverted(rgb: RGB) -> RGB {
    let mut new_rgb = [0u8; 3];
    for i in 0..=2 {
        new_rgb[i] = 255 - rgb[i]
    }
    new_rgb
}
/// Get luminance from RGB.
/// ### Examples
/// ```
/// let lum = Color::luminance([255, 255, 255]);
/// assert_eq!(1.0, lum);
/// ```
// https://www.w3.org/TR/WCAG20/#relativeluminancedef
pub fn luminance(rgb: RGB) -> f64 {
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
