struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let sz = grid.len();
        let mut in_degree = vec![0; sz];
        for i in 0..sz {
            for j in 0..sz {
                if i == j {
                    continue;
                }
                if grid[i][j] != 0 {
                    in_degree[i] += 1;
                }
            }
        }
        let mut max_val = 0;
        let mut ret = sz;
        for (i, &degree) in in_degree.iter().enumerate() {
            if degree > max_val {
                max_val = degree;
                ret = i;
            }
        }
        if ret == sz {
            -1
        } else {
            ret as _
        }
    }
}

fn main() {}
