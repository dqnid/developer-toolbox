use std::fmt::UpperHex;

use crate::{
    color::{ColorIntensity, HSL, RGB},
    core::ranged::BaseNumber,
};

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

impl UpperHex for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2X}{:0>2X}{:0>2X}", self.0, self.1, self.2)
    }
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    p
}

impl From<HSL> for RGB {
    fn from(color: HSL) -> Self {
        let h = color.0.to_f32() / 360.0;
        let s = color.1.to_f32() / 100.0;
        let l = color.2.to_f32() / 100.0;

        // No saturation
        if color.1 == 0 {
            let shade = l * 255.0;
            let intensity = ColorIntensity::new(shade as i16);
            return Self(intensity.clone(), intensity.clone(), intensity.clone());
        }

        let temp_1: f32 = if l < 0.5 {
            // Low lum
            l * (1.0 + s)
        } else {
            // High lum
            l + s - l * s
        };

        let temp_2: f32 = 2.0 * l - temp_1;

        let red = hue_to_rgb(temp_2, temp_1, h + 1.0 / 3.0);
        let green = hue_to_rgb(temp_2, temp_1, h);
        let blue = hue_to_rgb(temp_2, temp_1, h - 1.0 / 3.0);

        Self::new(
            (red * 255.0).round() as u8,
            (green * 255.0).round() as u8,
            (blue * 255.0).round() as u8,
        )
    }
}
