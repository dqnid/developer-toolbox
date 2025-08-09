use super::*;

use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

impl<const LOW: BaseNumber, const HIGH: BaseNumber> PartialEq<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    fn eq(&self, other: &BaseNumber) -> bool {
        self.0 == *other
    }

    fn ne(&self, other: &BaseNumber) -> bool {
        self.0 != *other
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> PartialOrd<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    fn partial_cmp(&self, other: &BaseNumber) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    fn ge(&self, other: &BaseNumber) -> bool {
        self.0 >= *other
    }

    fn le(&self, other: &BaseNumber) -> bool {
        self.0 <= *other
    }

    fn gt(&self, other: &BaseNumber) -> bool {
        self.0 > *other
    }

    fn lt(&self, other: &BaseNumber) -> bool {
        self.0 < *other
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Add<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    type Output = BaseNumber;
    fn add(self, rhs: BaseNumber) -> Self::Output {
        self.0 + rhs
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Sub<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    type Output = BaseNumber;
    fn sub(self, rhs: BaseNumber) -> Self::Output {
        self.0 - rhs
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Div<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    type Output = BaseNumber;
    fn div(self, rhs: BaseNumber) -> Self::Output {
        self.0 / rhs
    }
}

impl<const LOW: BaseNumber, const HIGH: BaseNumber> Mul<BaseNumber>
    for RangedInt<{ LOW }, { HIGH }>
{
    type Output = BaseNumber;
    fn mul(self, rhs: BaseNumber) -> Self::Output {
        self.0 * rhs
    }
}
