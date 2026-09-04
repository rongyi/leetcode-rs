struct Solution;

mod ai {
    struct Solution;
    use std::collections::HashMap;

    impl Solution {
        pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
            let mut prefix_sum = 0;
            let mut count = 0;
            let mut map = HashMap::new();
            map.insert(0, 1); // Empty subarray with sum 0

            for &num in &nums {
                prefix_sum += num;

                // Check if there's a prefix sum that would give us the goal
                // prefix_sum - goal = previous_prefix_sum
                if let Some(&freq) = map.get(&(prefix_sum - goal)) {
                    count += freq;
                }

                *map.entry(prefix_sum).or_insert(0) += 1;
            }

            count
        }
    }
}

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let sz = nums.len();
        let mut left = 0;
        let mut cur_sum = 0;
        let mut ret = 0;
        for i in 0..sz {
            cur_sum += nums[i];
            while left < i && cur_sum > goal {
                cur_sum -= nums[left];
                left += 1;
            }
            if cur_sum < goal {
                continue;
            }
            if cur_sum == goal {
                ret += 1;
            }
            let mut t = left;
            while t < i && nums[t] == 0 {
                t += 1;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
