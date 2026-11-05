struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut has_lower = false;
        let mut has_upper = false;

        for c in word.chars().skip(1) {
            if c.is_ascii_lowercase() {
                has_lower = true;
            } else if c.is_ascii_uppercase() {
                has_upper = true;
            }
        }

        if has_lower && has_upper {
            return false;
        }

        if word.len() > 1 && word.chars().skip(1).all(|c| c.is_uppercase()) {
            return word.chars().next().unwrap().is_uppercase();
        }

        true
    }
}

fn main() {}
