#[cfg(test)]
#[path = "./test/colors.test.rs"]
mod test;

pub mod hsl;
pub mod rgb;

use std::fmt::{UpperHex, write};

use crate::core::ranged::RangedInt;

pub type ColorIntensity = RangedInt<0, 255>;
pub type ColorHue = RangedInt<0, 360>;
pub type Percentage = RangedInt<0, 100>;
#[derive(Debug)]
pub struct RGB(ColorIntensity, ColorIntensity, ColorIntensity);
#[derive(Debug)]
pub struct HSL(ColorHue, Percentage, Percentage);
// pub struct HSV(ColorHue, Percentage, Percentage);
#[derive(Debug)]
pub struct Color(RGB);

impl Color {
    pub fn format(&self) -> String {
        format!("{:?}", self.0)
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
