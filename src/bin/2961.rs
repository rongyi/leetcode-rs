struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let n = variables.len();
        let mut res = Vec::new();
        for i in 0..n {
            let v = &variables[i];
            let (a, b, c, m) = (v[0], v[1], v[2], v[3]);
            // ((a^b % 10)^c) % m == target
            let mut pow1 = 1;
            for _ in 0..b {
                pow1 = (pow1 * a) % 10;
            }
            let mut pow2 = 1;
            for _ in 0..c {
                pow2 = (pow2 * pow1) % m;
            }
            if pow2 == target {
                res.push(i as i32);
            }
        }
        res
    }
}

fn main() {}
