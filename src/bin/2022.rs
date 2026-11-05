struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let sz = original.len();
        let m = m as usize;
        let n = n as usize;
        if m * n != sz {
            return vec![];
        }
        let mut ret = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                ret[i][j] = original[i * n + j];
            }
        }

        ret
    }
}

fn main() {}
