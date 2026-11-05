struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let sz = s.len();

        let mut i = 0;
        let mut j = sz - 1;

        while i < j {
            while i < j && !s[i].is_ascii_alphabetic() {
                i += 1;
            }
            while i < j && !s[j].is_ascii_alphabetic() {
                j -= 1;
            }
            s.swap(i, j);
            i += 1;
            j -= 1;
        }

        s.into_iter().collect()
    }
}

fn main() {}
