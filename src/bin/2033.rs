struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        for row in grid {
            for num in row {
                nums.push(num);
            }
        }
        nums.sort();

        let median = nums[nums.len() / 2];
        let mut operations = 0;

        for num in nums {
            let diff = (num - median).abs();
            if diff % x != 0 {
                return -1;
            }
            operations += diff / x;
        }

        operations
    }
}

fn main() {}
