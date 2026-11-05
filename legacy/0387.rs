struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = [0; 26];
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if count[c as usize - 'a' as usize] == 1 {
                return i as i32;
            }
        }


        -1
    }
}

fn main() {}
