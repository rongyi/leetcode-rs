#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, usize> = HashMap::new();
        for &num in nums.iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut max_freq = 0;
        for &v in cnt.values() {
            max_freq = max_freq.max(v);
        }
        let mut max_nums = Vec::new();
        for (&k, &v) in cnt.iter() {
            if v == max_freq {
                max_nums.push(k);
            }
        }
        let mut ret = nums.len();

        for &num in max_nums.iter() {
            let mut start = 0;
            for i in 0..nums.len() {
                if nums[i] == num {
                    start = i;
                    break;
                }
            }
            let mut end = nums.len() - 1;
            for j in (0..nums.len()).rev() {
                if nums[j] == num {
                    end = j;
                    break;
                }
            }
            ret = ret.min(end - start + 1);
        }

        ret as i32
    }
}

fn main() {
    let input = [1, 2, 2, 3, 1].into_iter().collect();
    let val = Solution::find_shortest_sub_array(input);
    println!("{}", val);
}
