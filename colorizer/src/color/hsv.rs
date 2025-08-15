use regex::Regex;
use std::fmt::Display;

use crate::color::{ColorHue, HSV, Percentage, RGB};

impl HSV {
    pub fn new(h: u16, s: u8, v: u8) -> Self {
        Self(
            ColorHue::new(h as i16),
            Percentage::new(s as i16),
            Percentage::new(v as i16),
        )
    }
}

impl Display for HSV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsv({}, {}%, {}%)", self.0, self.1, self.2)
    }
}

impl PartialEq for HSV {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1 || self.2 != other.2
    }
}

// TODO: manage error
impl From<String> for HSV {
    fn from(value: String) -> Self {
        let regex = Regex::new(r"hsv\(([0-9]+),([0-9]+),([0-9 ]+)\)").unwrap();
        let numbers = value.replace(" ", "").replace("%", "");
        let result = regex.captures(&numbers);

        match result {
            Some(value_list) => {
                // Numeric
                let h: u16 = value_list[1].parse::<u16>().unwrap();
                let s: u8 = value_list[2].parse::<u8>().unwrap();
                let v: u8 = value_list[3].parse::<u8>().unwrap();
                return Self::new(h, s, v);
            }
            None => (),
        }

        Self::new(0, 0, 0)
    }
}

impl From<RGB> for HSV {
    fn from(color: RGB) -> Self {
        let r = color.0.to_f32() / 255.0;
        let g = color.1.to_f32() / 255.0;
        let b = color.2.to_f32() / 255.0;

        let min: f32 = r.min(g.min(b));
        let max: f32 = r.max(g.max(b));

        let v = max;

        if min == max {
            return Self::new(0, 0, v as u8);
        }

        let s = (max - min) / max;
        let dif = max - min;
        let rc = (max - r) / dif;
        let gc = (max - g) / dif;
        let bc = (max - b) / dif;

        let mut h: f32;
        if r == max {
            h = bc - gc;
        } else if g == max {
            h = 2.0 + rc - bc;
        } else {
            h = 4.0 + gc - rc;
        }

        h = (h / 6.0).rem_euclid(1.0);

        Self::new(
            (h * 360.0).round() as u16,
            (s * 100.0).round() as u8,
            (v * 100.0).round() as u8,
        )
    }
}
