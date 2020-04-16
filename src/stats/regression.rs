use super::distribution::*;

pub trait Regression {
    type Output: Distribution;

    fn slope(&self, x: &[f64], y: &[f64]) -> Self::Output;
}

pub struct LeastSquare;

impl Regression for LeastSquare {
    type Output = Normal;

    // OLS estimator converges in distribution to standard normal random variable
    // by Lindeberg Central Limit Theorem.
    fn slope(&self, x: &[f64], y: &[f64]) -> Normal {
        let xx = x.iter().fold(0.0, |s, v| s + v * v);
        let xy = x.iter().zip(y.iter()).fold(0.0, |s, v| s + v.0 * v.1);
        let slope = xy / xx;

        let e2 = x
            .iter()
            .zip(y.iter())
            .fold(0.0, |s, v| s + (v.1 - v.0 * slope).powi(2));
        let mut var = e2 / (xx * (x.len() - 1) as f64);

        // bias correction
        var *= 2.0;

        Normal::new(slope, var.sqrt())
    }
}
