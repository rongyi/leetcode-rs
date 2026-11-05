struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = 0;
        let mut bar_start = false;

        for &b in s.iter() {
            if b == b'|' {
                bar_start = !bar_start;
            }
            if b == b'*' && !bar_start {
                cnt += 1;
            }
        }

        cnt
    }
}

fn main() {}
