struct Solution;

impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let mut last: Vec<i64> = vec![0; 26];
        let mut ret: i64 = 0;

        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - 'a' as u8) as usize;
            last[idx] = (i + 1) as i64;
            for &cur in last.iter() {
                ret += cur;
            }
        }

        ret
    }
}

fn main() {}
