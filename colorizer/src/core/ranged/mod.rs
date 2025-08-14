use std::fmt::{Display, UpperHex};

mod foreign_operations;
mod self_operations;

pub type BaseNumber = i16;

#[derive(Eq, Clone, Debug)]
pub struct RangedInt<const LOW: BaseNumber, const HIGH: BaseNumber>(BaseNumber);

impl<const LOW: BaseNumber, const HIGH: BaseNumber> RangedInt<{ LOW }, { HIGH }> {
    pub const LOW: BaseNumber = LOW;
    pub const HIGH: BaseNumber = HIGH;

    pub fn new(number: BaseNumber) -> Self {
        Self(number.min(Self::HIGH).max(Self::LOW))
    }

    pub fn to_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Display for RangedInt<{ LOW }, { HIGH }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> UpperHex for RangedInt<{ LOW }, { HIGH }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}
