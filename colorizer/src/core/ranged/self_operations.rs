use super::*;

use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

impl<const LOW: BaseNumber, const HIGH: BaseNumber> PartialEq for RangedInt<{ LOW }, { HIGH }> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> PartialOrd for RangedInt<{ LOW }, { HIGH }> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Add for RangedInt<{ LOW }, { HIGH }> {
    type Output = BaseNumber;
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Sub for RangedInt<{ LOW }, { HIGH }> {
    type Output = BaseNumber;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Div for RangedInt<{ LOW }, { HIGH }> {
    type Output = BaseNumber;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Mul for RangedInt<{ LOW }, { HIGH }> {
    type Output = BaseNumber;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0
    }
}
