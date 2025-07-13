use crate::solutions::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let fx = x as f64;
        let guess = fx / 2.0;
        let result = (1.0 / 2.0) * (guess + fx / guess);

        Self::reduce_sqrt(fx, result).floor() as i32
    }

    pub fn reduce_sqrt(x: f64, y: f64) -> f64 {
        let result = (1.0 / 2.0) * (y + x / y);
        let deviation = y - result;

        if deviation < 0.05 {
            return result;
        }

        Self::reduce_sqrt(x, result)
    }
}
