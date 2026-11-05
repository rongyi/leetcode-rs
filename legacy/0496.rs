
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let mut ret: Vec<i32> = Vec::new();
        for num in nums2 {
            while !stack.is_empty() && *stack.last().unwrap() < num {
                cache.insert(stack.pop().unwrap(), num);
            }
            stack.push(num);
        }

        for num in nums1 {
            ret.push(*cache.get(&num).unwrap_or(&-1));
        }

        ret
    }
}

fn main() {
    let nums1 = [4, 1, 2];
    let nums2 = [1, 3, 4, 2];
    Solution::next_greater_element(nums1.into(), nums2.into());
}
