struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let mut ret = s.clone();
            let sz = s.len();
            let s = s.repeat(2);

            for i in 0..sz {
                let rotated: String = (&s[i..i + sz]).chars().collect();
                if rotated < ret {
                    ret = rotated;
                }
            }

            ret
        } else {
            let mut chars: Vec<char> = s.chars().collect();

            chars.sort_unstable();
            chars.into_iter().collect()
        }
    }
}

fn main() {}
