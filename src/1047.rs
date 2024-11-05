struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stk: Vec<char> = Vec::new();
        for c in s.chars() {
            if stk.is_empty() {
                stk.push(c);
            } else {
                if c == *stk.last().unwrap() {
                    stk.pop();
                } else {
                    stk.push(c);
                }
            }
        }

        stk.into_iter().collect()
    }
}

fn main() {}
