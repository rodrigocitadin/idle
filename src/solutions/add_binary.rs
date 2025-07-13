use crate::solutions::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut i, mut j, mut carry) = (a.len() as i32 - 1, b.len() as i32 - 1, 0);
        let mut res = String::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let sum = carry
                + if i >= 0 {
                    a.as_bytes()[i as usize] - b'0'
                } else {
                    0
                }
                + if j >= 0 {
                    b.as_bytes()[j as usize] - b'0'
                } else {
                    0
                };

            res.push((b'0' + sum % 2) as char);
            carry = sum / 2;
            i -= 1;
            j -= 1;
        }

        res.chars().rev().collect()
    }
}
