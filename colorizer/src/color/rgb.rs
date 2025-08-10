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
