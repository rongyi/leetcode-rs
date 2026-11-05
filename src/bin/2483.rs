struct Solution;

use std::i32;
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let s = customers.as_bytes();
        let sz = s.len();
        let mut acc = 0;
        // include current index
        let mut suffix_y = vec![0; sz];
        let mut acc = 0;
        for i in (0..sz).rev() {
            if s[i] == b'Y' {
                acc += 1;
            }
            suffix_y[i] = acc;
        }

        acc = 0;
        // not include itself
        let mut prefix_n = vec![0; sz];
        for i in 0..sz {
            prefix_n[i] = acc;
            if s[i] == b'N' {
                acc += 1;
            }
        }
        let mut min_val = acc;
        let mut min_idx = sz;

        for i in (0..sz).rev() {
            let cur = prefix_n[i] + suffix_y[i];
            if cur <= min_val {
                min_idx = i;
                min_val = cur;
            }
        }

        min_idx as _
    }
}

fn main() {}
