#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut count = vec![0; 26];

        for i in 0..s.len() {
            let diff = if t_bytes[i] >= s_bytes[i] {
                (t_bytes[i] - s_bytes[i]) as i32
            } else {
                (t_bytes[i] + 26 - s_bytes[i]) as i32
            };

            if diff > 0 {
                let moves_needed = diff + 26 * count[diff as usize];
                if moves_needed > k {
                    return false;
                }
                count[diff as usize] += 1;
            }
        }

        true
    }
}

fn main() {}
