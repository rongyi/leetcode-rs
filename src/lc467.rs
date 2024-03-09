struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        // i8 in case underflow
        let s: Vec<i8> = s.chars().map(|c| (c as u8 - 'a' as u8) as i8).collect();
        let mut max_len = 0;
        // calculate the way end with current char
        // e.g. zab
        // for b we have 3 substring end with b and in this infinite mega string
        // that is: b ab zab so 3 is count for number of substring *end with* b
        // and for a we have a za, that is 2
        // and z for only one
        // so total way is 1 + 2 + 3
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

fn main() {
    Solution::find_substring_in_wrapround_string("a".to_string());
}
