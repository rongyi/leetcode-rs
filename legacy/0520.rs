struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut have_lower = false;
        let mut upper = 0;
        for c in word.chars() {
            if c.is_lowercase() {
                if upper > 1 {
                    return false;
                }
                have_lower = true;
            } else {
                upper += 1;
                if have_lower {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {}
