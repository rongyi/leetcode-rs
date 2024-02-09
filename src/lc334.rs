struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut small = i32::MAX;
        let mut big = i32::MAX;
        for num in nums {
            if num <= small {
                small = num;
            } else if num <= big {
                big = num;
            } else {
                return true;
            }
        }
        false
    }
}

fn main() {}
