use super::functions::{erf, inverf};
use std::f64::consts::SQRT_2;

pub trait Distribution {
    fn mean(&self) -> f64;

    fn median(&self) -> f64;

    fn variance(&self) -> f64;

    unsafe fn cdf_unchecked(&self, x: f64) -> f64;

    fn cdf(&self, x: f64) -> f64 {
        let (l, r) = self.domain();
        if x < l || r < x {
            panic!("Domain error: {} is out of domain ({}, {})", x, l, r);
        }

        unsafe { self.cdf_unchecked(x) }
    }

    unsafe fn icdf_unchecked(&self, prob: f64) -> f64;

    fn icdf(&self, prob: f64) -> f64 {
        if prob < 0.0 || 1.0 < prob {
            panic!("Domain error: {} is out of domain (0.0, 1.0)", prob);
        }

        unsafe { self.icdf_unchecked(prob) }
    }

    #[inline]
    fn domain(&self) -> (f64, f64) {
        (std::f64::NEG_INFINITY, std::f64::INFINITY)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Normal {
    mu: f64,
    sigma: f64,
}

impl Normal {
    #[inline]
    pub const fn new(mu: f64, sigma: f64) -> Normal {
        Normal { mu, sigma }
    }
}

impl Distribution for Normal {
    #[inline]
    fn mean(&self) -> f64 {
        self.mu
    }

    #[inline]
    fn median(&self) -> f64 {
        self.mu
    }

    #[inline]
    fn variance(&self) -> f64 {
        self.sigma * self.sigma
    }

    #[inline]
    unsafe fn cdf_unchecked(&self, x: f64) -> f64 {
        (1.0 + erf((x - self.mu) / (self.sigma * SQRT_2))) / 2.0
    }

    #[inline]
    unsafe fn icdf_unchecked(&self, prob: f64) -> f64 {
        let z = SQRT_2 * inverf(2.0 * prob - 1.0);
        self.mu + self.sigma * z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn normal_distribution() {
        let normal = Normal::new(0.0, 1.0);
        assert_float_absolute_eq!(normal.cdf(0.0), 0.5);
        assert_float_absolute_eq!(normal.cdf(1.6448536269514724), 0.95);
        assert_float_absolute_eq!(normal.cdf(1.959963984540054), 0.975);
        assert_float_absolute_eq!(normal.cdf(2.5758293035489004), 0.995);
        assert_float_absolute_eq!(normal.icdf(0.5), 0.0);
        assert_float_absolute_eq!(normal.icdf(0.95), 1.6448536269514724);
        assert_float_absolute_eq!(normal.icdf(0.975), 1.959963984540054);
        assert_float_absolute_eq!(normal.icdf(0.995), 2.5758293035489004);
    }
}
