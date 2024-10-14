struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.is_empty() {
            return 1;
        }

        let n = n as usize;
        let mut trust_count = vec![0; n + 1];
        let mut trusted_by = vec![0; n + 1];

        for p in trust.iter() {
            let a = p[0] as usize;
            let b = p[1] as usize;
            trust_count[a] += 1;
            trusted_by[b] += 1;
        }

        for i in 1..=n {
            if trust_count[i] == 0 && trusted_by[i] == n - 1 {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {}
