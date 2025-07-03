use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let matching = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    for c in s.chars() {
        if matching.contains_key(&c) {
            if stack.pop() != matching.get(&c).copied() {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}
