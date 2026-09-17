struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + nums[i] as usize);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }

        true
    }
}

fn main() {
    let input = vec![3, 2, 1, 0, 4];
    let v = Solution::can_jump(input);
    println!("{v}");
}
