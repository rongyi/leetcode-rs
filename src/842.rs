#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        let num: Vec<char> = num.chars().collect();
        Self::backtrack(&num, 0, &mut ret);

        ret
    }

    fn backtrack(s: &Vec<char>, start: usize, ret: &mut Vec<i32>) -> bool {
        if start >= s.len() && ret.len() >= 3 {
            return true;
        }
        if start >= s.len() {
            return false;
        }

        let max_split = if s[start] == '0' { 1 } else { 10 };

        for i in 1..=max_split {
            if start + i > s.len() {
                break;
            }

            let chunk: String = s[start..start + i].iter().collect();
            let val = chunk.parse::<i64>().unwrap();
            if val > i32::MAX as i64 {
                continue;
            }
            let cur_len = ret.len();

            if cur_len >= 2 && ret[cur_len - 1] as i64 + ret[cur_len - 2] as i64 != val {
                continue;
            }
            ret.push(val as i32);
            if Self::backtrack(s, start + i, ret) {
                return true;
            }

            ret.pop();
        }

        false
    }
}

fn main() {}
