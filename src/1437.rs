#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_one: Option<usize> = None;
        let mut cur_one: Option<usize> = None;

        for (i, &val) in nums.iter().enumerate() {
            if val == 1 {
                if prev_one.is_none() {
                    if cur_one.is_none() {
                        cur_one = Some(i);
                    } else {
                        prev_one = cur_one;
                        // we should check interval
                        cur_one = Some(i);
                        if (cur_one.unwrap() - prev_one.unwrap()) as i32 <= k {
                            return false;
                        }
                    }
                } else {
                    prev_one = cur_one;
                    cur_one = Some(i);

                    if (cur_one.unwrap() - prev_one.unwrap()) as i32 <= k {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {}
