pub fn is_palindrome(x: i32) -> bool {
    let chars: Vec<char> = x.to_string().chars().collect();
    let len = chars.len();
    let mid = len / 2;

    for i in 0..mid {
        let j = len - 1 - i;
        if chars[i] != chars[j] {
            return false;
        }
    }

    return true;
}
