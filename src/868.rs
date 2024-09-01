struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let s = format!("{:b}", n);
        let mut prev: Option<usize> = None;
        let mut ret = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                if let Some(j) = prev {
                    ret = ret.max(i - j);
                }
                prev = Some(i);
            }
        }
        ret as i32
    }
}

fn main() {}
