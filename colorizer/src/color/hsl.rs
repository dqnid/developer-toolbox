use crate::color::{ColorHue, HSL, Percentage, RGB};

impl HSL {
    pub fn new(h: u16, s: u8, l: u8) -> Self {
        Self(
            ColorHue::new(h as i16),
            Percentage::new(s as i16),
            Percentage::new(l as i16),
        )
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
