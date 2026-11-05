struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted: Vec<i32> = vec![-1; nums.len() + 1];
        let mut target = -1;
        let mut pos = -1;
        for num in nums.into_iter() {
            if sorted[num as usize] != -1 {
                target = num;
            }
            sorted[num as usize] = num;
        }
        for i in 1..sorted.len() {
            if sorted[i] == -1 {
                pos = i as i32;
                break;
            }
        }

        vec![target, pos]
    }
}

fn main() {}
