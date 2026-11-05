struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let sz = nums.len();
        let mut ret = vec![-1; sz];

        for i in 0..(2 * sz) {
            let idx = i % sz;
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[idx] {
                ret[stack.pop().unwrap()] = nums[idx];
            }

            if i < sz {
                stack.push(i);
            }
        }

        ret
    }
}

fn main() {}
