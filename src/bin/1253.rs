#![allow(dead_code)]

struct Solution;

impl Solution {
    // upper and lower is a bit confuse, just row0 sum and row1 sum
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colums: Vec<i32>) -> Vec<Vec<i32>> {
        let sz = colums.len();
        let mut ret = vec![vec![0; sz]; 2];
        let mut alter_index = vec![];

        for j in 0..sz {
            if colums[j] == 2 {
                ret[0][j] = 1;
                ret[1][j] = 1;
                upper -= 1;
                lower -= 1;

                if upper < 0 || lower < 0 {
                    return vec![];
                }
            } else if colums[j] == 1 {
                alter_index.push(j);
            }
        }
        if (upper + lower) != alter_index.len() as i32 {
            return vec![];
        }

        for i in 0..upper as usize {
            ret[0][alter_index[i] as usize] = 1;
        }

        for i in upper as usize..alter_index.len() {
            ret[1][alter_index[i]] = 1;
        }

        ret
    }
}

fn main() {}
