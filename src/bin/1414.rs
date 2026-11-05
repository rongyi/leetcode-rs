#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibos = vec![1, 1];
        while fibos[fibos.len() - 1] < k {
            let cur = fibos[fibos.len() - 1] + fibos[fibos.len() - 2];
            fibos.push(cur);
        }
        let mut count = 0;
        let mut remain = k;
        for &val in fibos.iter().rev() {
            if remain == 0 {
                break;
            }

            if val <= remain {
                remain -= val;
                count += 1;
            }
        }

        count
    }
}

fn main() {}
