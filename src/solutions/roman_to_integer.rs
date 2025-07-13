use crate::solutions::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let numbers: Vec<i32> = s.chars().map(value).collect();

        numbers.iter().enumerate().fold(0, |acc, (i, &num)| {
            if i + 1 < numbers.len() && num < numbers[i + 1] {
                acc - num
            } else {
                acc + num
            }
        })
    }
}
