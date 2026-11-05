struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        let first = &strs[0];

        for (i, c) in first.chars().enumerate() {
            for rest in strs.iter().skip(1) {
                if i >= rest.len() || rest.chars().nth(i).unwrap() != c {
                    return prefix;
                }
            }
            // all rest have c, so we push c to prefix
            prefix.push(c);
        }

        prefix
    }
}
