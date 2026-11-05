struct Solution;

impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        // 1. Find the minimum element in the array
        let &min_val = nums.iter().min().unwrap();

        // 2. Check if there's any element that, when modded by min_val,
        // produces a non-zero remainder (a new, smaller minimum).
        for &num in &nums {
            if num % min_val != 0 {
                // If we find a smaller non-zero value, we can reduce
                // the entire array to a single element.
                return 1;
            }
        }

        // 3. If all elements are multiples of min_val,
        // we count how many times min_val appears.
        let count = nums.iter().filter(|&&x| x == min_val).count();

        // The result is the ceiling of count / 2
        ((count + 1) / 2) as i32
    }
}

fn main() {}
