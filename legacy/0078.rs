struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut cur = Vec::new();

        Self::backtrack(0, &nums, &mut cur, &mut ret);

        ret
    }

    fn backtrack(start: usize, nums: &Vec<i32>, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        ret.push(cur.clone());

        for i in start..nums.len() {
            cur.push(nums[i]);
            Self::backtrack(i + 1, nums, cur, ret);

            cur.pop();
        }
    }
}
