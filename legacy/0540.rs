struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let is_even_halves = (right - mid) % 2 == 0;
            if nums[mid] == nums[mid + 1] {
                if is_even_halves {
                    left = mid + 2;
                } else {
                    right = mid - 1;
                }
            } else if nums[mid] == nums[mid - 1] {
                if is_even_halves {
                    right = mid - 2;
                } else {
                    left = mid + 1;
                }
            } else {
                return nums[mid];
            }
        }
        nums[left]
    }
}

fn main() {
    let s = "00".parse::<i32>().unwrap();
    println!("{}", s);
}
