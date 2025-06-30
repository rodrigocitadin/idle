use std::collections::HashMap;

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

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;
    let mut i = 0;

    while i < chars.len() {
        let current = value(chars[i]);
        let next = chars.get(i + 1).map(|&c| value(c));

        if let Some(n) = next {
            if current < n {
                result += n - current;
                i += 2;
                continue;
            }
        }

        result += current;
        i += 1;
    }

    return result;
}
