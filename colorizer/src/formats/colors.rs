use std::convert::From;

use crate::core::ranged::{BaseNumber, RangedInt};

pub type ColorIntensity = RangedInt<0, 255>;
pub type ColorHue = RangedInt<0, 360>;
pub type Percentage = RangedInt<0, 100>;
#[derive(Debug)]
pub struct RGB(ColorIntensity, ColorIntensity, ColorIntensity);
pub struct HSL(ColorHue, Percentage, Percentage);
// pub struct HSV(ColorHue, Percentage, Percentage);

#[derive(Debug)]
pub struct Color(RGB);

impl Color {
    pub fn format(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(
            ColorIntensity::new(r as BaseNumber),
            ColorIntensity::new(g as BaseNumber),
            ColorIntensity::new(b as BaseNumber),
        )
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1 || self.2 != other.2
    }
}

impl HSL {
    pub fn new(h: u16, s: u8, l: u8) -> Self {
        Self(
            ColorHue::new(h as i16),
            Percentage::new(s as i16),
            Percentage::new(l as i16),
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

fn min_of_float_vec(vector: Vec<f32>) -> Option<f32> {
    let mut min: Option<f32> = None;

    for element in vector.iter() {
        if let Some(value) = min {
            if element < &value {
                min = Some(*element)
            }
        } else {
            min = Some(*element);
        }
    }

    min
}

fn max_of_float_vec(vector: Vec<f32>) -> Option<f32> {
    let mut max: Option<f32> = None;

    for element in vector.iter() {
        if let Some(value) = max {
            if element > &value {
                max = Some(*element)
            }
        } else {
            max = Some(*element);
        }
    }

    max
}

impl From<RGB> for HSL {
    fn from(value: RGB) -> Self {
        let r = value.0.to_f32() / 255.0;
        let g = value.1.to_f32() / 255.0;
        let b = value.2.to_f32() / 255.0;

        let min: f32 = min_of_float_vec(vec![r, g, b]).unwrap();
        let max: f32 = max_of_float_vec(vec![r, g, b]).unwrap();

        // Luminance
        let l = ((min + max) / 2.0).round();

        // Saturation
        let s: f32;
        if r == g && g == b {
            s = 0.0;
        } else {
            if l <= 0.5 {
                s = (max - min) / (max + min);
            } else {
                s = (max - min) / (2.0 - max - min);
            }
        }

        // Hue
        let h: f32;
        if max == r {
            h = (g - b) / (max - min);
        } else if max == g {
            h = 2.0 + (b - r) / (max - min);
        } else {
            h = 4.0 + (r - g) / (max - min);
        }

        HSL::new(
            (h * 60.0).round() as u16,
            (s * 100.0).round() as u8,
            (l * 100.0).round() as u8,
        )
    }
}

impl From<HSL> for RGB {
    fn from(color: HSL) -> Self {
        // No saturation
        if color.1 == 0 {
            let shade = color.2 * 255 / 100;
            let intensity = ColorIntensity::new(shade);
            return Self(intensity.clone(), intensity.clone(), intensity.clone());
        }

        let temp_1: f32;

        if color.2 < 50 {
            // Low lum
            temp_1 = (color.2.to_f32() / 100.0) * (color.1.to_f32() / 100.0 + 1.0);
        } else {
            // High lum
            temp_1 = (color.2.to_f32() / 100.0 + color.1.to_f32())
                - (color.2.to_f32() * color.1.to_f32());
        }

        let temp_2: f32 = (color.2.to_f32() / 100.0) * 2.0 - temp_1;

        let hue = color.0.to_f32() / 360.0;

        let mut temp_r = hue + 0.333;
        let temp_g = hue;
        let temp_b = hue - 0.333;

        // Normalize values
        if temp_r > 1.0 {
            temp_r = temp_r - 1.0;
        }
        if temp_b < 0.0 {
            temp_r = temp_r + 1.0;
        }

        // Calc Red
        let red: f32;
        if temp_r * 6.0 < 1.0 {
            red = temp_2 + (temp_1 - temp_2) * 6.0 * temp_r;
        } else if temp_r * 2.0 < 1.0 {
            red = temp_1;
        } else if temp_r * 3.0 < 2.0 {
            red = temp_2 + (temp_1 - temp_2) * (0.666 - temp_r) * 6.0;
        } else {
            red = temp_2;
        }

        // Calc Green
        let green: f32;
        if temp_g * 6.0 < 1.0 {
            green = temp_2 + (temp_1 - temp_2) * 6.0 * temp_g;
        } else if temp_g * 2.0 < 1.0 {
            green = temp_1;
        } else if temp_g * 3.0 < 2.0 {
            green = temp_2 + (temp_1 - temp_2) * (0.666 - temp_g) * 6.0;
        } else {
            green = temp_2;
        }

        // Calc blue
        let blue: f32;
        if temp_b * 6.0 < 1.0 {
            blue = temp_2 + (temp_1 - temp_2) * 6.0 * temp_b;
        } else if temp_b * 2.0 < 1.0 {
            blue = temp_1;
        } else if temp_b * 3.0 < 2.0 {
            blue = temp_2 + (temp_1 - temp_2) * (0.666 - temp_b) * 6.0;
        } else {
            blue = temp_2;
        }

        Self::new(
            (red * 255.0).round() as u8,
            (green * 255.0).round() as u8,
            (blue * 255.0).round() as u8,
        )
    }
}
