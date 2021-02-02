// mod convert;
#![allow(unused_imports)]

use crate::convert::Convert;

#[test]
fn RGB_Hex() {
    let rgb_to_hex = Convert::rgb_to_hex([255, 77, 128]);
    let hex_to_rgb = Convert::hex_to_rgb("ff4d80").unwrap();
    let rgba_to_hex = Convert::rgba_to_hex([0, 0, 255, 255]);
    let hex_to_rgba = Convert::hex_to_rgb("0000ffff").unwrap();
    assert_eq!(rgb_to_hex, "ff4d80");
    assert_eq!(rgba_to_hex, "0000ffff");
    assert_eq!(hex_to_rgb, vec![255, 77, 128]);
    assert_eq!(hex_to_rgba, vec![0, 0, 255, 255]);
}

#[test]
fn RGB_HSV() { // X
    let rgb_to_hsv = Convert::rgb_to_hsv([128, 255, 77]);
    println!("{:#?}", rgb_to_hsv);
    let hsv_to_rgb = Convert::hsv_to_rgb(rgb_to_hsv);
    println!("{:#?}", hsv_to_rgb);
}
