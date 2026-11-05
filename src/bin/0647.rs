struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut ret = 0;
        for i in 0..sz {
            let i = i as i32;
            let mut j = 0;
            // i as center
            while i - j >= 0 && i + j < sz as i32 && s[(i + j) as usize] == s[(i - j) as usize] {
                ret += 1;
                j += 1;
            }
            let mut j = 0;
            // i - 1, i as center
            while i - 1 - j >= 0
                && i + j < sz as i32
                && s[(i - 1 - j) as usize] == s[(i + j) as usize]
            {
                j += 1;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
