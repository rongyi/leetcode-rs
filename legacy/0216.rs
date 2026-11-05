struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();

        Self::backtrack(k, n, 1, &mut cur, &mut ret);

        ret
    }

    fn backtrack(k: i32, target: i32, index: i32, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if target < 0 || cur.len() > k as usize {
            return;
        }
        if target == 0 && cur.len() == k as usize {
            ret.push(cur.clone());
        }
        for i in index..=9 {
            cur.push(i);
            Self::backtrack(k, target - i, i + 1, cur, ret);
            cur.pop();
        }
    }
}

fn main() {}
