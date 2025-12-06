struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ret = 0;
        for i in 1..=n {
            let cur = i * i;
            let s: Vec<char> = cur.to_string().chars().collect();
            if Self::check(&s, 0, i) {
                ret += cur;
            }
        }

        ret
    }
    fn check(s: &[char], pos: usize, num: i32) -> bool {
        if pos == s.len() && num == 0 {
            return true;
        }
        if num < 0 {
            return false;
        }
        let mut acc = 0;
        for i in pos..s.len() {
            acc = acc * 10 + (s[i] as u8 - '0' as u8) as i32;
            if Self::check(s, i + 1, num - acc) {
                return true;
            }
        }

        false
    }
}

fn main() {}
