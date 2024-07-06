#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut ret: Vec<i32> = vec![i32::MAX; sz];

        let mut prev: Option<usize> = None;
        for i in 0..sz {
            if s[i] == c {
                ret[i] = 0;
                prev = Some(i);
            } else {
                if let Some(prev) = prev {
                    ret[i] = ret[i].min((i - prev) as i32);
                }
            }
        }
        let mut after: Option<usize> = None;
        for i in (0..sz).rev() {
            if s[i] == c {
                ret[i] = 0;
                after = Some(i);
            } else {
                if let Some(after) = after {
                    ret[i] = ret[i].min((after - i) as i32);
                }
            }
        }

        ret
    }
}

fn main() {}
