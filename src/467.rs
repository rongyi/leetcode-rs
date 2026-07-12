struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut max_len = 0;
        let mut count = vec![0; 26];
        for i in 0..s.len() {
            if i > 0 && (s[i - 1] - s[i] == 25 || s[i] - s[i - 1] == 1) {
                max_len += 1;
            } else {
                max_len = 1;
            }
            count[s[i] as usize] = count[s[i] as usize].max(max_len);
        }

        // count.iter().fold(0, |acc, &cur| acc + cur)
        count.into_iter().sum()
    }
}

fn main() {}
