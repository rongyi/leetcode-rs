struct Solution;

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut i = 1;
        while i < sz && nums[i] > nums[i - 1] {
            i += 1;
        }
        // perfect
        if i == sz {
            return 0;
        }
        // from nums[i] must still be ordered
        for j in i + 1..sz {
            if nums[j] < nums[j - 1] {
                return -1;
            }
        }
        // valid case
        if nums[sz - 1] < nums[0] {
            return (sz - i) as i32;
        }
        -1
    }
}

fn main() {}
