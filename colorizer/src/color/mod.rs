#[cfg(test)]
#[path = "./test/colors.test.rs"]
mod test;

pub mod hsl;
pub mod hsv;
pub mod rgb;

use std::fmt::UpperHex;

use regex::Regex;

use crate::core::ranged::RangedInt;

pub type ColorIntensity = RangedInt<0, 255>;
pub type ColorHue = RangedInt<0, 360>;
pub type Percentage = RangedInt<0, 100>;
#[derive(Debug)]
pub struct RGB(ColorIntensity, ColorIntensity, ColorIntensity);
#[derive(Debug)]
pub struct HSL(ColorHue, Percentage, Percentage);
#[derive(Debug)]
pub struct HSV(ColorHue, Percentage, Percentage);
#[derive(Debug)]
pub struct Color(RGB);

impl Color {
    pub fn try_parse(input: String) -> Result<Color, ()> {
        let input = input.replace(" ", "").to_uppercase();

        let hex_regex = Regex::new(r".*(#[a-fA-F0-9]{3,6}).*").unwrap();
        let rgb_regex =
            Regex::new(r"(rgb\([ ]*[0-9]+[ ]*,[ ]*[0-9]+[ ]*,[ ]*[0-9 ]+[ ]*\)).*").unwrap();
        let hsl_regex =
            Regex::new(r"(hsl\([ ]*[0-9]+[ ]*,[0-9]+[ ]*[%]*[ ]*,[0-9 ]+[ ]*[%]*[ ]*\)).*")
                .unwrap();
        let hsv_regex =
            Regex::new(r"(hsv\([ ]*[0-9]+[ ]*,[0-9]+[ ]*[%]*[ ]*,[0-9 ]+[ ]*[%]*[ ]*\)).*")
                .unwrap();

        let hex_result = hex_regex.captures(&input);
        match hex_result {
            Some(color) => return Ok(Color::from(RGB::from(color[1].to_string()))),
            None => (),
        }

        let rgb_result = rgb_regex.captures(&input);
        match rgb_result {
            Some(color) => return Ok(Color::from(RGB::from(color[1].to_string()))),
            None => (),
        }

        let hsl_result = hsl_regex.captures(&input);
        match hsl_result {
            Some(color) => return Ok(Color::from(HSL::from(color[1].to_string()))),
            None => (),
        }

        let hsv_result = hsv_regex.captures(&input);
        match hsv_result {
            Some(color) => return Ok(Color::from(HSV::from(color[1].to_string()))),
            None => (),
        }

        Err(())
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
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

impl From<HSV> for Color {
    fn from(color: HSV) -> Self {
        Color(RGB::from(color))
    }
}
