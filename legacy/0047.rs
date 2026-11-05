struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut cur = Vec::new();
        let mut visited = vec![false; nums.len()];
        let mut ret: Vec<Vec<i32>> = Vec::new();

        Self::backtract(&nums, &mut cur, &mut ret, &mut visited);

        ret
    }

    fn backtract(
        nums: &[i32],
        cur: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if cur.len() == nums.len() {
            ret.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            // the tricky part, only allow one pass
            // for same number
            if visited[i] || (i > 0 && nums[i] == nums[i - 1] && !visited[i - 1]) {
                continue;
            }

            visited[i] = true;
            cur.push(nums[i]);
            Self::backtract(nums, cur, ret, visited);

            visited[i] = false;
            cur.pop();
        }

    }
}
