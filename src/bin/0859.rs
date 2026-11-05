struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let a = s.as_bytes();
        let b = goal.as_bytes();
        if a.len() != b.len() {
            return false;
        }
        let sz = a.len();
        let mut swapped = false;
        let mut i = 0;
        while i < sz {
            if a[i] == b[i] {
                i += 1;
                continue;
            }
            if swapped {
                return false;
            }
            let mut j = i + 1;
            while j < sz && a[j] == b[j] {
                j += 1;
            }
            // only one diff of index i
            if j == sz {
                return false;
            }
            if a[i] == b[j] && a[j] == b[i] {
                swapped = true;
            } else {
                //  can not swap
                // two diff and can not swap
                return false;
            }
            i = j + 1;
        }
        let mut cnt: HashMap<u8, usize> = HashMap::new();
        a.iter()
            .copied()
            .for_each(|c| *cnt.entry(c).or_default() += 1);

        swapped || cnt.values().any(|&v| v > 1)
    }
}

fn main() {}
