use crate::Solution;

impl Solution {
    //
    // Fib solution but rust doesn't have tail call optimization
    //
    
    // pub fn climb_stairs(n: i32) -> i32 {
    //     match n {
    //         1 => 1,
    //         2 => 2,
    //         _ => Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2),
    //     }
    // }

    //
    // Fib without recursion
    //
    
    // pub fn climb_stairs(n: i32) -> i32 {
    //     if n == 1 {
    //         return 1;
    //     }

    //     let (mut a, mut b) = (1, 2);
    //     for _ in 3..=n {
    //         let temp = a + b;
    //         a = b;
    //         b = temp;
    //     }

    //     b
    // }

    //
    // Binet’s Formula
    //
    
    pub fn climb_stairs(n: i32) -> i32 {
        let sqrt5 = 5f64.sqrt();
        let phi = (1.0 + sqrt5) / 2.0;
        let psi = (1.0 - sqrt5) / 2.0;

        ((phi.powf((n + 1) as f64) - psi.powf((n + 1) as f64)) / sqrt5 + 0.5).floor() as i32
    }
}
