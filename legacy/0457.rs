struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut nums: Vec<i32> = nums.into_iter().map(|i| i % sz as i32).collect();
        for i in 0..sz {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = i;
            while nums[slow] * nums[fast] > 0
                && nums[slow] * nums[Self::next_index(&nums, fast)] > 0
            {
                slow = Self::next_index(&nums, slow);
                fast = Self::next_index(&nums, Self::next_index(&nums, fast));
                if slow == fast {
                    if slow == Self::next_index(&nums, slow) {
                        break;
                    }
                    return true;
                }
            }
            let mut j = i;
            while nums[j] * nums[Self::next_index(&nums, j)] > 0 {
                let tmp = j;
                j = Self::next_index(&nums, j);
                nums[tmp] = 0;
            }
        }
        false
    }
    fn next_index(nums: &[i32], k: usize) -> usize {
        ((k as i32 + nums[k as usize] + nums.len() as i32) % nums.len() as i32) as usize
    }
}

fn main() {}
