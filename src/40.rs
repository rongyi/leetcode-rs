struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut cur = vec![];
        let mut out: Vec<Vec<i32>> = vec![];

        Self::try_combi_sum(&candidates, &mut cur, target, 0, &mut out);

        out
    }

    fn try_combi_sum(
        candidates: &Vec<i32>,
        cur: &mut Vec<i32>,
        target: i32,
        pos: usize,
        out: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            out.push(cur.clone());
            return;
        }

        for i in pos..candidates.len() {
            // ignore this duplicate case
            // case1. use current
            if candidates[i] > target {
                break;
            }

            // this is crucial
            if i > pos && candidates[i] == candidates[i - 1] {
                continue;
            }

            cur.push(candidates[i]);
            Self::try_combi_sum(candidates, cur, target - candidates[i], i + 1, out);
            cur.pop();
        }
    }
}

fn main() {}
