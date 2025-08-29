struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut stk: Vec<(i32, i32)> = Vec::new();
        stk.push((nums[sz - 1], 0));
        let mut ret = 0;
        for i in (0..sz - 1).rev() {
            let mut cnt = 0;
            while !stk.is_empty() && stk.last().unwrap().0 < nums[i] {
                cnt += 1;
                cnt = cnt.max(stk.last().unwrap().1);
                stk.pop();
            }
            ret = ret.max(cnt);
            stk.push((nums[i], cnt));
        }

        ret
    }
}

fn main() {
    let input = vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11];
    Solution::total_steps(input);
}
