struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cur: Vec<i32> = Vec::new();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut candidates = candidates;

        candidates.sort();
        Self::backtrack(&candidates, target, 0, &mut cur, &mut ret);

        ret
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        cur: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        } else if target == 0 {
            ret.push(cur.clone());
        } else {
            for i in start..candidates.len() {
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }
                cur.push(candidates[i]);
                Self::backtrack(candidates, target - candidates[i], i + 1, cur, ret);

                cur.pop();
            }
        }
    }
}
