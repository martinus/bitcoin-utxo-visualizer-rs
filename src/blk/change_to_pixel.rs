// Copyright 2016 Martin Ankerl.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Maps a blockheight & amount to a pixel.
pub struct ChangeToPixel {
    /// Height in pixels
    height: u32,

    // min satoshi will be lowest row, so height-1.
    min_satoshi: i64,

    // max satoshi will be highest row, so row 0.
    max_satoshi: i64,
}

pub struct LinearFunction {
    k: f64,
    d: f64,
}

impl LinearFunction {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> LinearFunction {
        let k = (y2 - y1) / (x2 - x1);
        let d = y1 - k * x1;

        LinearFunction { k: k, d: d }
    }

    pub fn calc(&self, x: f64) -> f64 {
        self.k * x + self.d
    }
}

impl ChangeToPixel {
    pub fn new(height: u32, min_satoshi: i64, max_satoshi: i64) -> ChangeToPixel {
        ChangeToPixel {
            height: height,
            min_satoshi: min_satoshi,
            max_satoshi: max_satoshi,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nearly_equal(a: f64, b: f64) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();

        if a == b {
            // Handle infinities.
            true
        } else if a == 0.0 || b == 0.0 || diff < std::f64::MIN_POSITIVE {
            // One of a or b is zero (or both are extremely close to it,) use absolute error.
            diff < (std::f64::EPSILON * std::f64::MIN_POSITIVE)
        } else {
            // Use relative error.
            (diff / f64::min(abs_a + abs_b, std::f64::MAX)) < std::f64::EPSILON
        }
    }

    #[test]
    fn linear_function() {
        let lf = LinearFunction::new(0.0, 0.0, 1.0, 1.0);
        assert!(nearly_equal(0.0, lf.calc(0.0)));
        assert!(nearly_equal(1.0, lf.calc(1.0)));
        assert!(nearly_equal(2.0, lf.calc(2.0)));

        let lf = LinearFunction::new(10.0, 2.0, 15.0, 7.0);
        assert!(nearly_equal(2.0, lf.calc(10.0)));
        assert!(nearly_equal(7.0, lf.calc(15.0)));

        assert!(nearly_equal(-8.0, lf.calc(0.0)));
        assert!(nearly_equal(0.0, lf.calc(8.0)));

        let lf = LinearFunction::new(10.0, 4.0, 15.0, 4.0);
        assert!(nearly_equal(4.0, lf.calc(0.0)));
        assert!(nearly_equal(4.0, lf.calc(10.0)));
        assert!(nearly_equal(4.0, lf.calc(-10.0)));

        let lf = LinearFunction::new(0.1, 1.1, 0.2, 0.8);
        assert!(nearly_equal(0.5, lf.calc(0.3)));
        assert!(nearly_equal(1.4, lf.calc(0.0)));
    }

}
