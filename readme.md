This is a module for rust programing language,To work with color.

## API
```rs
// Importing module
extern crate Color;

// Get luminance from RGB
let lum = Color::luminance([255, 255, 255]);
assert_eq!(1.0, lum);

// Create Color instance from hex value
let rgb = Color::from_hex("#00ff00").unwrap();

// Get RGB value.
assert_eq!([0, 255, 0], rgb.value());

// Create new Color instance
let mut rgb = Color::new([0, 0, 0]);

// Inverted rgb value
rgb.inverted();
assert_eq!([255, 255, 255], rgb.value());

// Convart rgb to hex value
let color_hex = rgb.to_hex();
assert_eq!("ffffff", color_hex);

// Get contrast ratio between two colors.
let contrast = rgb.contrast_ratio([0, 0, 0]);
assert_eq!(21.0, contrast);
```