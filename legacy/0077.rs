struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        let mut cur: Vec<i32> = Vec::new();
        Self::backtrack(1, n, k, &mut cur, &mut ret);

        ret
    }

    fn backtrack(i: i32, n: i32, k: i32, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if k == 0 {
            ret.push(cur.clone());
            return;
        }

        for j in i..=n {
            cur.push(j);
            Self::backtrack(j + 1, n, k - 1, cur, ret);
            cur.pop();
        }
    }
}
