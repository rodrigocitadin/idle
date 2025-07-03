pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = strs[0].chars().collect::<Vec<char>>();
    let mut prefix = String::new();

    for i in 0..first.len() {
        let c = first[i];
        for s in &strs {
            if s.chars().nth(i) != Some(c) {
                return prefix;
            }
        }

        prefix.push(c);
    }

    return prefix;
}
