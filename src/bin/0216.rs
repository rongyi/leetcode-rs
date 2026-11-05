struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();

        Self::backtrack(k as usize, n, 1, &mut cur, &mut ret);

        ret
    }

    fn backtrack(k: usize, target: i32, index: i32, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if target < 0 || cur.len() > k {
            return;
        }
        if target == 0 && cur.len() == k {
            ret.push(cur.clone());
            return;
        }
        for i in index..=9 {
            cur.push(i);
            // only use once
            Self::backtrack(k, target - i, i + 1, cur, ret);
            cur.pop();
        }
    }
}

fn main() {}
