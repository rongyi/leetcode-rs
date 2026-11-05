struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let s = s.as_bytes();
        let mut ret = 0;
        let sz = s.len();
        for (i, c) in s.iter().enumerate().skip(1) {
            if *c != s[i - 1] {
                ret += (i.min(sz - i)) as i64
            }
        }

        ret
    }
}

fn main() {}
