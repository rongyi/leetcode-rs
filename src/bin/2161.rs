struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut same_cnt = 0;
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for num in nums.into_iter() {
            if num == pivot {
                same_cnt += 1;
            } else if num < pivot {
                left.push(num);
            } else {
                right.push(num);
            }
        }
        left.extend(vec![pivot; same_cnt]);
        left.extend(right);

        left
    }
}

fn main() {}
