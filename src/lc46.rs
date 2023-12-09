struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cur = Vec::new();
        let mut ret = Vec::new();

        let mut visited = vec![false; nums.len()];

        Self::backtrack(&nums, &mut cur, &mut ret, &mut visited);

        ret
    }

    fn backtrack(
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
            if visited[i] {
                continue;
            }
            visited[i] = true;
            cur.push(nums[i]);
            Self::backtrack(nums, cur, ret, visited);

            visited[i] = false;
            cur.pop();
        }
    }
}
