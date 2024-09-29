struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let words: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        let mut ret = 0;
        let m = words.len();
        let n = words[0].len();
        for j in 0..n {
            for i in 1..m {
                if words[i][j] < words[i - 1][j] {
                    ret += 1;
                    break;
                }
            }
        }

        ret
    }
}

fn main() {}
