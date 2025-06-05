struct Solution;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = land.len();
        let n = land[0].len();
        let mut ret = Vec::new();

        for i in 0..m {
            for j in 0..n {
                //   0
                // 0 1 top left always have 0 up/left neibs
                if land[i][j] == 1
                    && (i == 0 || land[i - 1][j] == 0)
                    && (j == 0 || land[i][j - 1] == 0)
                {
                    let x1 = i;
                    let y1 = j;
                    let mut x2 = i;
                    let mut y2 = j;
                    while x2 < m && land[x2][j] == 1 {
                        x2 += 1;
                    }
                    x2 -= 1;

                    while y2 < n && land[i][y2] == 1 {
                        y2 += 1;
                    }
                    y2 -= 1;

                    ret.push(vec![x1 as i32, y1 as i32, x2 as i32, y2 as i32]);
                }
            }
        }

        ret
    }
}

fn main() {}
