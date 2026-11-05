struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let n = changed.len();
        if n % 2 != 0 {
            return vec![];
        }

        let mut freq = HashMap::new();

        for &num in &changed {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut changed = changed;
        changed.sort_unstable();

        let mut original = Vec::with_capacity(n / 2);

        for &num in &changed {
            if *freq.get(&num).unwrap_or(&0) == 0 {
                continue;
            }

            let double = num * 2;
            let mut need_delete: Option<i32> = None;
            if let Some(count) = freq.get_mut(&double) {
                if *count > 0 {
                    if num == 0 {
                        // Special case for 0 since 0*2 = 0
                        if *count >= 2 {
                            *count -= 2;
                            original.push(0);
                        } else {
                            return vec![];
                        }
                    } else {
                        *count -= 1;
                        need_delete = Some(num);
                        original.push(num);
                    }
                } else {
                    return vec![];
                }
            } else {
                return vec![];
            }
            // double mut ref hack
            if let Some(d) = need_delete {
                *freq.get_mut(&d).unwrap() -= 1;
            }
        }

        original
    }
}

fn main() {}
