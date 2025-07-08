pub fn length_of_last_word(s: String) -> i32 {
    let result: Vec<&str> = s.trim().split(" ").collect();
    result[result.len() - 1].len() as i32
}
