struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.is_empty() {
            return 1;
        }

        let n = n as usize;
        let mut out_count = vec![0; n + 1];
        let mut in_count = vec![0; n + 1];

        for p in trust.iter() {
            let a = p[0] as usize;
            let b = p[1] as usize;
            out_count[a] += 1;
            in_count[b] += 1;
        }

        for i in 1..=n {
            if out_count[i] == 0 && in_count[i] == n - 1 {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {}
