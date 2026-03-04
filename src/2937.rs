struct Solution;

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let prefix_min = s1.len().min(s2.len().min(s3.len()));
        let mut same_len = 0;
        for i in 0..prefix_min {
            if s1[i] == s2[i] && s2[i] == s3[i] {
                same_len += 1;
            } else {
                break;
            }
        }
        if same_len == 0 {
            return -1;
        }
        (s1.len() - same_len + s2.len() - same_len + s3.len() - same_len) as i32
    }
}

fn main() {}
