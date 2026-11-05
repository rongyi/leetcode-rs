#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut prefix = vec![0i64; sz + 1];
        let module = 1_000_000_007i64;
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num as i64;
        }
        let mut stack = Vec::new();
        // each val as min of subarry, it's right boudary
        let mut right = vec![sz; sz];

        for i in 0..sz {
            while !stack.is_empty() && nums[*stack.last().unwrap()] > nums[i] {
                // ok, we found a smaller brother, and mark it right index
                right[*stack.last().unwrap()] = i;
                stack.pop();
            }

            stack.push(i);
        }
        let mut left = vec![sz; sz];
        stack.clear();

        for i in (0..sz).rev() {
            // add equal compare to eliminate duplicate calculation
            while !stack.is_empty() && nums[*stack.last().unwrap()] >= nums[i] {
                left[*stack.last().unwrap()] = i;
                stack.pop();
            }

            stack.push(i);
        }

        let range_sum = |left_boudary: usize, right_bounary: usize| -> i64 {
            prefix[right_bounary + 1] - prefix[left_boudary]
        };

        let mut max_val = 0;

        for i in 0..sz {
            let l = if left[i] == sz { 0 } else { left[i] + 1 };
            let r = if right[i] == sz { sz - 1 } else { right[i] - 1 };
            let sum = range_sum(l, r);
            let min_product = sum * nums[i] as i64;
            max_val = max_val.max(min_product);
        }

        (max_val % module) as i32
    }
}

fn main() {}
