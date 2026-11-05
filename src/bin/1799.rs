#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        fn backtrack(
            nums: &[i32],
            mask: u16,
            ops_done: i32,
            memo: &mut HashMap<u16, i32>,
            gcds: &HashMap<(usize, usize), i32>,
        ) -> i32 {
            if ops_done * 2 == nums.len() as i32 {
                return 0;
            }

            if let Some(&result) = memo.get(&mask) {
                return result;
            }

            let mut max_score = 0;

            for i in 0..nums.len() {
                if mask & (1 << i) != 0 {
                    continue;
                }

                for j in i + 1..nums.len() {
                    if mask & (1 << j) != 0 {
                        continue;
                    }

                    let new_mask = mask | (1 << i) | (1 << j);
                    let gcd_val = *gcds.get(&(i, j)).unwrap();
                    let score = (ops_done + 1) * gcd_val;

                    let next_score = score + backtrack(nums, new_mask, ops_done + 1, memo, gcds);
                    max_score = max_score.max(next_score);
                }
            }

            memo.insert(mask, max_score);
            max_score
        }

        let n = nums.len();
        let mut gcds = HashMap::new();

        // Precompute all GCD values
        for i in 0..n {
            for j in i + 1..n {
                gcds.insert((i, j), gcd(nums[i], nums[j]));
            }
        }

        let mut memo = HashMap::new();
        backtrack(&nums, 0, 0, &mut memo, &gcds)
    }
}

fn main() {}
