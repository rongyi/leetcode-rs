struct Solution;

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let sz = nums.len();
        let mut right = vec![sz; sz];
        let mut left = vec![-1; sz];
        let mut s: Vec<usize> = vec![]; // as stack

        for i in 0..sz {
            while !s.is_empty() && nums[i] < nums[*s.last().unwrap()] {
                right[*s.last().unwrap()] = i;
                s.pop();
            }
            s.push(i);
        }
        s.clear();

        for i in (0..sz).rev() {
            while !s.is_empty() && nums[i] < nums[*s.last().unwrap()] {
                left[*s.last().unwrap()] = i as i32;
                s.pop();
            }
            s.push(i);
        }
        for i in 0..sz {
            let len = right[i] as i64 - left[i] as i64 - 1;
            if nums[i] as i64 * len > threshold as i64 {
                return len as _;
            }
        }

        -1
    }
}

fn main() {}
