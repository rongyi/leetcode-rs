
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut cur = Vec::new();

        let mut nums = nums;
        nums.sort();

        Self::backtrack(&nums, 0, &mut cur, &mut ret);

        ret
    }

    fn backtrack(nums: &Vec<i32>, start: usize, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        ret.push(cur.clone());

        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            cur.push(nums[i]);

            Self::backtrack(nums, i + 1, cur, ret);

            cur.pop();
        }
    }
}
