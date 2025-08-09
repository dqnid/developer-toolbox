use std::convert::From;

use crate::core::ranged::{BaseNumber, RangedInt};

type ColorIntensity = RangedInt<0, 255>;
struct RGB(ColorIntensity, ColorIntensity, ColorIntensity);
struct HSL(RangedInt<0, 360>, RangedInt<0, 100>, RangedInt<0, 100>);
struct HSV(RangedInt<0, 360>, RangedInt<0, 100>, RangedInt<0, 100>);

pub struct Color(RGB);

impl Color {}

impl RGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(
            ColorIntensity::new(r as BaseNumber),
            ColorIntensity::new(g as BaseNumber),
            ColorIntensity::new(b as BaseNumber),
        )
    }
}

impl From<RGB> for Color {
    fn from(color: RGB) -> Self {
        Self(color)
    }
}

impl From<HSL> for Color {
    fn from(color: HSL) -> Self {
        Color(RGB::from(color))
    }
}

impl From<HSL> for RGB {
    fn from(color: HSL) -> Self {
        // No saturation
        if color.1 == 0 {
            let shade = color.2 / 100 * 255;
            let intensity = ColorIntensity::new(shade);
            return Self(intensity.clone(), intensity.clone(), intensity.clone());
        }

        let mut temp_1: i16 = 0;

        if color.2 < 50 {
            // Low lum
            temp_1 = color.2.clone() * (color.1 + 100);
        } else {
            // High lum
            temp_1 = color.2.clone() + color.1.clone() - color.2.clone() * color.1.clone();
        }

        let temp_2: i16 = color.2 * 2 - temp_1;
        let hue = (color.0.clone() / 360) * 100;

        Self::new(0, 0, 0)
    }
}
