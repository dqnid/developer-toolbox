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

impl PartialEq for HSL {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1 || self.2 != other.2
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

        let min: f32 = r.min(g.min(b));
        let max: f32 = r.max(g.max(b));

        let h;
        let s;
        // Luminance set
        let l = (min + max) / 2.0;

        if max == min {
            s = 0.0;
            h = 0.0;
        } else {
            // Saturation set
            if l <= 0.5 {
                s = (max - min) / (max + min);
            } else {
                s = (max - min) / (2.0 - max - min);
            }

            // Hue set
            // TODO FIX
            if max == r {
                let temp = if g < b { 6.0 } else { 0.0 };
                h = (g - b) / (max - min) + temp;
            } else if max == g {
                h = (b - r) / (max - min) + 2.0;
            } else {
                h = (r - g) / (max - min) + 4.0;
            }
        }

        HSL::new(
            (h / 6.0 * 360.0).round() as u16,
            (s * 100.0).round() as u8,
            (l * 100.0).round() as u8,
        )
    }
}
