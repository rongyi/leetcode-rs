struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut s: Vec<char> = s.chars().collect();
        let mut ret: Vec<String> = Vec::new();

        Self::recur(&mut s, &mut ret, 0);

        ret
    }

    fn recur(s: &mut Vec<char>, ret: &mut Vec<String>, i: usize) {
        if i == s.len() {
            ret.push(s.iter().collect());
            return;
        }
        // don't change case
        Self::recur(s, ret, i + 1);
        // change case
        if s[i].is_alphabetic() {
            let origin = s[i];
            let case_trans = if s[i].is_lowercase() {
                s[i].to_ascii_uppercase()
            } else {
                s[i].to_ascii_lowercase()
            };
            s[i] = case_trans;
            Self::recur(s, ret, i + 1);

            // restore back
            s[i] = origin;
        }
    }
}

fn main() {}
