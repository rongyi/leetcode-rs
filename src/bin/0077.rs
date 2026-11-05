struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut cur = vec![];
        let mut out = vec![];

        Self::backtrack(1, n, k, &mut cur, &mut out);
        // let mut visited: Vec<bool> = vec![false; n as usize + 1];
        // Self::backtrack_permutation(n, k, &mut cur, &mut out, &mut visited);

        out
    }

    fn backtrack(start: i32, n: i32, k: i32, cur: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if k == 0 {
            out.push(cur.clone());
            return;
        }

        for i in start..=n {
            cur.push(i);
            Self::backtrack(i + 1, n, k - 1, cur, out);
            cur.pop();
        }
    }

    fn backtrack_permutation(
        n: i32,
        k: i32,
        cur: &mut Vec<i32>,
        out: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if k == 0 {
            out.push(cur.clone());
            return;
        }

        for i in 1..=n {
            if visited[i as usize] {
                continue;
            }
            cur.push(i);
            visited[i as usize] = true;

            Self::backtrack_permutation(n, k - 1, cur, out, visited);

            cur.pop();
            visited[i as usize] = false;
        }
    }
}

fn main() {
    let v = Solution::combine(4, 2);
    println!("{:?}", v);
}
