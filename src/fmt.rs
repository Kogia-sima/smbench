use std::fmt;

pub struct Time {
    second: f64,
    significants: u16,
}

#[inline]
pub fn time(second: f64) -> Time {
    Time {
        second,
        significants: 5,
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = self.second;
        let abs_value = value.abs();
        let suffix = if abs_value < 1e-9 {
            value *= 1e12;
            " ps"
        } else if abs_value < 1e-6 {
            value *= 1e9;
            " ns"
        } else if abs_value < 1e-3 {
            value *= 1e6;
            " us"
        } else if abs_value < 1.0 {
            value *= 1e3;
            " ms"
        } else {
            " s"
        };

        fmt_float(f, value, self.significants, suffix)
    }
}

pub struct Bytes {
    bytes: usize,
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = self.bytes;
        let suffix = if bytes < 1_000 {
            " B"
        } else if bytes < 1_000_000 {
            bytes /= 1_000;
            " KB"
        } else if bytes < 1_000_000_000 {
            bytes /= 1_000_000;
            " MB"
        } else {
            bytes /= 1_000_000_000;
            " GB"
        };

        let string = format!("{}{}", bytes, suffix);
        f.pad(&string)
    }
}

fn fmt_float(f: &mut fmt::Formatter, value: f64, significants: u16, suffix: &str) -> fmt::Result {
    let string = if value.is_finite() {
        let abs_value = value.abs();
        let mut c = 1.0;
        let mut count = 0;
        while c <= abs_value {
            c *= 10.0;
            count += 1;
        }

        let precision = significants.saturating_sub(count);
        format!("{:.*}{}", precision as usize, value, suffix)
    } else {
        let value_str = if value.is_nan() {
            "NaN"
        } else if value == std::f64::INFINITY {
            if f.sign_plus() {
                "+inf"
            } else {
                "inf"
            }
        } else if value == std::f64::NEG_INFINITY {
            "inf"
        } else {
            ""
        };

        format!("{}{}", value_str, suffix)
    };

    f.pad(&string)
}

#[cfg(test)]
mod tests {
    use super::time;

    #[test]
    fn format_time() {
        assert_eq!(format!("{}", time(1.0)), "1.0000 s");
        assert_eq!(format!("{}", time(0.99999)), "999.99 ms");
        assert_eq!(format!("{}", time(3.14159265358979e-6)), "3.1416 us");
        assert_eq!(format!("{}", time(-1e-9)), "-1.0000 ns");
        assert_eq!(format!("{}", time(1e-11)), "10.000 ps");
        assert_eq!(format!("{:>10}", time(1.23)), "  1.2300 s");
        assert_eq!(format!("{:>10}", time(-2.71828e-3)), "-2.7183 ms");
    }
}
