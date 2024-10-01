struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut strs: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        let m = strs.len();
        let n = strs[0].len();
        let mut ret = 0;
        for j in 0..n {
            let mut i = 1;
            let mut has_equal = false;
            while i < m {
                let prev: String = strs[i - 1][0..=j].iter().copied().collect();
                let cur: String = strs[i][0..=j].iter().copied().collect();
                if prev > cur {
                    ret += 1;
                    break;
                } else if prev == cur {
                    has_equal = true;
                }

                i += 1;
            }
            // no need to see afterward, this column is already sorted
            if i == m && !has_equal {
                return ret;
            }
            if i < m {
                for k in 0..m {
                    strs[k][j] = '*';
                }
            }
        }

        ret
    }
}

fn main() {}
