use crate::color_types::*;
pub struct Convert;

impl Convert {

    pub fn rgb_to_hwb(rgb: RGB) -> [f64; 3] {
        let r = rgb[0] as f64;
        let g = rgb[1] as f64;
        let b = rgb[2] as f64;
        let h = Convert::rgb_to_hsl(rgb)[0];
        let w = 1.0 / 255.0 * r.min((g).min(b));
        let b = 1.0 - 1.0 / 255.0 * r.max((g).max(b));

        [h, w * 100.0, b * 100.0]
    }
    pub fn rgb_to_cmyk(rgb: RGB) -> [f64; 4] {
        let r = rgb[0] as f64 / 255.0;
        let g = rgb[1] as f64 / 255.0;
        let b = rgb[2] as f64 / 255.0;
        let k = (1.0 - r).min(1.0 - g).min(1.0 - b);
        let c = (1.0 - r - k) / (1.0 - k);
        let m = (1.0 - g - k) / (1.0 - k);
        let y = (1.0 - b - k) / (1.0 - k);
        [c * 100.0, m * 100.0, y * 100.0, k * 100.0]
    }
    pub fn rgb_to_xyz(rgb: RGB) -> [f64; 3] {
        let mut r = rgb[0] as f64 / 255.0;
        let mut g = rgb[1] as f64 / 255.0;
        let mut b = rgb[2] as f64 / 255.0;

        r = if r > 0.04045 {
            ((r + 0.055) / 1.055).powf(2.4)
        } else {
            r / 12.92
        };
        g = if g > 0.04045 {
            ((g + 0.055) / 1.055).powf(2.4)
        } else {
            g / 12.92
        };
        b = if b > 0.04045 {
            ((b + 0.055) / 1.055).powf(2.4)
        } else {
            b / 12.92
        };
        let x = (r * 0.4124564) + (g * 0.3575761) + (b * 0.1804375);
        let y = (r * 0.2126729) + (g * 0.7151522) + (b * 0.072175);
        let z = (r * 0.0193339) + (g * 0.119192) + (b * 0.9503041);
        [x * 100.0, y * 100.0, z * 100.0]
    }
    pub fn rgb_to_lab(rgb: RGB) -> [f64; 3] {
        let [mut x, mut y, mut z] = Convert::rgb_to_xyz(rgb);
        x /= 95.047;
        y /= 100.0;
        z /= 108.883;
        x = if x > 0.008856 {
            x.powf(1.0 / 3.0)
        } else {
            (7.787 * x) + (16 / 116) as f64
        };
        y = if y > 0.008856 {
            y.powf(1.0 / 3.0)
        } else {
            (7.787 * y) + (16 / 116) as f64
        };
        z = if z > 0.008856 {
            z.powf(1.0 / 3.0)
        } else {
            (7.787 * z) + (16 / 116) as f64
        };
        let l = (116.0 * y) - 16.0;
        let a = 500.0 * (x - y);
        let b = 200.0 * (y - z);
        [l, a, b]
    }
    
    pub fn hsl_to_hsv(hsl: [f64; 3]) -> [f64; 3] {
        let h = hsl[0];
        let mut s = hsl[1] / 100.0;
        let mut l = hsl[2] / 100.0;
        let mut smin = s;
        let lmin = l.max(0.01);

        l *= 2.0;
        s *= if l <= 1.0 { l } else { 2.0 - l };
        smin *= if lmin <= 1.0 { lmin } else { 2.0 - lmin };
        let v = (l + s) / 2.0;
        let sv = if l == 0.0 {
            (2.0 * smin) / (lmin + smin)
        } else {
            (2.0 * s) / (l + s)
        };

        [h, sv * 100.0, v * 100.0]
    }


    pub fn rgb_to_hsl(rgb: RGB) -> [f64; 3] {
        let r = rgb[0] as f64 / 255.0;
        let g = rgb[1] as f64 / 255.0;
        let b = rgb[2] as f64 / 255.0;
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let delta = max - min;
        let mut h = 0.0;
        let mut s = 0.0;
        let l = (max + min) / 2.0;
        if delta != 0.0 {
            s = delta / (1.0 - (2.0 * l - 1.0).abs());
            h = if max == r {
                ((g - b) / delta) % 6.0
            } else if max == g {
                (b - r) / delta + 2.0
            } else {
                (r - g) / delta + 4.0
            };
        }
        h = (h * 60.0).round();
        if h < 0.0 {
            h += 360.0;
        }
        [h, s * 100.0, l * 100.0]
    }
    pub fn hsl_to_rgb(hsl: [f64; 3]) -> [f64; 3] {
        let h = hsl[0] / 360.0;
        let s = hsl[1] / 100.0;
        let l = hsl[2] / 100.0;

        let mut t3;
        let mut val;
        if s == 0.0 {
            val = l * 255.0;
            return [val, val, val];
        }

        let t2 = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let t1 = 2.0 * l - t2;

        let mut rgb = [0f64; 3];
        for i in 0..3 {
            t3 = h + 1.0 / 3.0 * -(i as f64 - 1.0);
            if t3 < 0.0 {
                t3 += 1.0;
            }
            if t3 > 1.0 {
                t3 -= 1.0;
            }
            val = if 6.0 * t3 < 1.0 {
                t1 + (t2 - t1) * 6.0 * t3
            } else if 2.0 * t3 < 1.0 {
                t2
            } else if 3.0 * t3 < 2.0 {
                t1 + (t2 - t1) * (2.0 / 3.0 - t3) * 6.0
            } else {
                t1
            };
            rgb[i] = val * 255.0;
        }
        rgb
    }

   
    pub fn rgb_to_hsv(rgb: RGB) -> HSV {
        let rdif;
        let gdif;
        let bdif;
        let mut h = 0.0;
        let s;

        let r = rgb[0] as f64 / 255.0;
        let g = rgb[1] as f64 / 255.0;
        let b = rgb[2] as f64 / 255.0;
        let v = r.max(g).max(b);
        let diff = v - r.min(g).min(b);
        let diffc = |c| (v - c) / 6.0 / diff + 1.0 / 2.0;

        if diff == 0.0 {
            h = 0.0;
            s = 0.0;
        } else {
            s = diff / v;
            rdif = diffc(r);
            gdif = diffc(g);
            bdif = diffc(b);

            if r == v {
                h = bdif - gdif;
            } else if g == v {
                h = (1.0 / 3.0) + rdif - bdif;
            } else if b == v {
                h = (2.0 / 3.0) + gdif - rdif;
            }

            if h < 0.0 {
                h += 1.0;
            } else if h > 1.0 {
                h -= 1.0;
            }
        }
        [h * 360.0, s * 100.0, v * 100.0]
    }
    pub fn hsv_to_rgb(hsv: HSV) -> RGB {
        let h = hsv[0] as f64 / 60.0;
        let s = hsv[1] as f64 / 100.0;
        let mut v = hsv[2] as f64 / 100.0;
        let hi = h.floor() % 6.0;

        let f = h - h.floor();
        let p = 255.0 * v * (1.0 - s);
        let q = 255.0 * v * (1.0 - (s * f));
        let t = 255.0 * v * (1.0 - (s * (1.0 - f)));
        v *= 255.0;

        if hi == 0.0 {
            [v as u8, t as u8, p as u8]
        } else if hi == 1.0 {
            [q as u8, v as u8, p as u8]
        } else if hi == 2.0 {
            [p as u8, v as u8, t as u8]
        } else if hi == 3.0 {
            [p as u8, q as u8, v as u8]
        } else if hi == 4.0 {
            [t as u8, p as u8, v as u8]
        } else {
            [v as u8, p as u8, q as u8]
        }
    }

    /// Convart Hex value to RBG value.
    /// ### Examples
    /// ```
    /// Color::hex_to_rgb("#00ff00").unwrap();
    /// ```
    pub fn hex_to_rgb(hex: &str) -> Result<Vec<u8>, &str> {
        let hex = hex.trim_start_matches("#");
        let mut rgb: Vec<u8> = Vec::new();
        let length = hex.len();
        if length == 6 || length == 8 {
            for i in 0..(length / 2) {
                let x = i * 2;
                match u8::from_str_radix(&hex[x..(x + 2)], 16) {
                    Ok(color) => rgb.push(color),
                    Err(_) => return Err("Invalid Digit"),
                }
            }
            Ok(rgb)
        } else {
            Err("Invalid Color")
        }
    }
    /// Convart RGB value to Hex value.
    pub fn rgb_to_hex(rgb: RGB) -> String {
        format!("{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
    }
    /// Convart (RGBA or ARGB) value to Hex value.
    pub fn rgba_to_hex(rgb: RGBA) -> String {
        format!("{:02x}{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2], rgb[3])
    }
}
