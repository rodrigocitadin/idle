use crate::solutions::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut chars = strs[0].chars();
        let mut prefix = String::new();

        for (i, c) in chars.enumerate() {
            if strs.iter().any(|s| s.chars().nth(i) != Some(c)) {
                break;
            }

            prefix.push(c);
        }

        return prefix;
    }
}
