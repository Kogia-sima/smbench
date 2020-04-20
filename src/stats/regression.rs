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
        var = correct_bias(slope, var);

        Normal::new(slope, var.sqrt())
    }
}

fn correct_bias(best: f64, var: f64) -> f64 {
    let ratio = var.sqrt() / best;
    var * (1.0 - 0.4 * ratio.ln()).powi(2)
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;

    #[test]
    fn correct_bias_test() {
        use super::correct_bias;
        assert_float_relative_eq!(correct_bias(1.0, 1.0), 1.0);
        assert_float_relative_eq!(correct_bias(100.0, 10000.0), 10000.0);
        assert_float_relative_eq!(correct_bias(1000.0, 1.0), 14.160937502274603);
        assert_float_relative_eq!(correct_bias(100000.0, 10.0), 264.6745621272857);
    }
}
